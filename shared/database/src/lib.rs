use std::cmp::max;
use std::time::{Duration, Instant};

use clap::{Args, ValueEnum};
use colored::Colorize;
pub use redis;
use redis::Cmd;
pub use sea_orm;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
pub use sea_query;
use thiserror::Error;
use tokio::{join, try_join};
use tracing::log::LevelFilter;
use tracing::{debug, info, instrument, warn};

use common::error_stack::ResultExt;
use migration::{Migrator, MigratorTrait};
pub use orm::prelude::*;
pub use orm::*;

mod batch;
pub mod orm;

#[derive(Debug, Clone, Args)]
pub struct DatabaseParams {
    /// Strategy to keep the database schema up to date.
    #[clap(
        value_enum,
        long = "database-migration-strategy",
        default_value = "apply",
        env = "DATABASE_MIGRATION_STRATEGY"
    )]
    migration_strategy: MigrationStrategy,

    #[clap(
        value_enum,
        long = "database-migration-failure-strategy",
        default_value = "error",
        env = "DATABASE_MIGRATION_FAILURE_STRATEGY"
    )]
    migration_failure_strategy: MigrationFailureStrategy,

    /// Database URL.
    #[clap(long = "database-url", env = "DATABASE_URL")]
    database_url: String,

    #[clap(
        long = "database-min-connections",
        env = "DATABASE_MIN_CONNECTIONS",
        default_value = "1"
    )]
    database_min_connections: u32,

    #[clap(
        long = "database-max-connections",
        env = "DATABASE_MAX_CONNECTIONS",
        default_value = "15"
    )]
    database_max_connections: u32,

    /// Database URL for read-only operations. if not set the default database url will be used.
    /// The target database must be a replica of the main database.
    #[clap(long = "replica-database-url", env = "DATABASE_URL_RO")]
    database_url_ro: Option<String>,

    #[clap(
        long = "database-max-ro-connections",
        env = "DATABASE_MAX_RO_CONNECTIONS",
        default_value = "5"
    )]
    database_max_ro_connections: u32,

    #[clap(
        long = "database-min-ro-connections",
        env = "DATABASE_MIN_RO_CONNECTIONS",
        default_value = "1"
    )]
    database_min_ro_connections: u32,

    #[clap(long = "skip-connection-test", env = "DATABASE_SKIP_CONNECTION_TEST")]
    skip_connection_test: bool,

    #[clap(long = "redis-url", env = "REDIS_URL")]
    redis_url: String,
}

#[derive(Debug, Default, Clone, Copy, ValueEnum)]
pub enum MigrationStrategy {
    ///when the database is not up to date,
    ///it will be upgraded using the included migration scripts.
    ///when the database is up to date, nothing will be done.
    #[default]
    #[value(aliases = ["default", "upgrade"])]
    Apply,
    ///drop the database and create it from scratch. this is useful for development and testing.
    ///TO NOT USE IN PRODUCTION. **ALL** DATA WILL BE LOST.
    #[value(aliases = ["fresh", "reset", "drop_and_create"])]
    Fresh,
    ///do not apply any migration. it is up to the administrator to ensure the database scheme is uptodate.
    /// We remcommend to use this if you are using a serverless strategy with short-lived instances to redice startup time.
    /// Operating missmatch between database schema and application expected schema is undefined behavior.
    /// DO NOT USE THIS WITH Auto Updating Applications.
    #[value(aliases = ["none", "skip", "ignore"])]
    NoOp,

    /// If the database schema is not up-to-date, the application will fail to start.
    #[value(aliases = ["fail", "error", "abort", "assert"])]
    Fail,
}

#[derive(Debug, Default, Clone, Copy, ValueEnum)]
pub enum MigrationFailureStrategy {
    /// Continue without logging any warning if the database migration fails.
    Ignore,
    /// Continue but log a warning if the database migration fails.
    Warn,
    /// Fail the application startup if the database migration fails. This is the default behavior.
    #[default]
    Error,
}

#[derive(Debug, Error)]
pub enum InitDatabaseError {
    #[error("cannot connect to the database")]
    ConnectToDatabase,
    #[error("cannot migrate the database")]
    MigrateDatabase,
}

#[derive(Debug, Clone)]
pub struct DatabaseInstance {
    pub db: DatabaseConnection,
    pub db_ro: Option<DatabaseConnection>,
    pub redis: redis::Client,
}

#[instrument(level = "debug", skip(params), fields(
    url = % params.database_url,
    ro_url = ? params.database_url_ro,
    migration = ? params.migration_strategy,
))]
pub async fn init_database(
    params: &DatabaseParams,
) -> error_stack::Result<DatabaseInstance, InitDatabaseError> {
    let (database, database_ro, redis) = try_join!(
        init_rw_database(params),
        init_ro_database(params),
        init_redis(params),
    )?;

    Ok(DatabaseInstance {
        db: database,
        db_ro: database_ro,
        redis,
    })
}

async fn init_rw_database(
    params: &DatabaseParams,
) -> error_stack::Result<DatabaseConnection, InitDatabaseError> {
    debug!("database url: {}", params.database_url.bright_blue());
    info!("connecting to the database...");
    let time = Instant::now();
    let min_connections = max(params.database_min_connections, 1);
    let max_connections = max(params.database_max_connections, min_connections);

    let options = ConnectOptions::new(&params.database_url)
        .max_connections(max_connections)
        .min_connections(min_connections)
        .test_before_acquire(!params.skip_connection_test)
        .sqlx_logging_level(LevelFilter::Debug)
        .sqlx_slow_statements_logging_settings(LevelFilter::Warn, Duration::from_secs(5 * 60))
        .to_owned();
    let database = Database::connect(options)
        .await
        .change_context(InitDatabaseError::ConnectToDatabase)?;
    let time = time.elapsed();
    info!("database connected ({:?})", time);

    let time = Instant::now();
    let migrations_result = apply_migrations(&database, params.migration_strategy)
        .await
        .change_context(InitDatabaseError::MigrateDatabase);
    if let Err(report) = migrations_result {
        match params.migration_failure_strategy {
            MigrationFailureStrategy::Ignore => {}
            MigrationFailureStrategy::Warn => {
                warn!("database migration failed: {}", report);
            }
            MigrationFailureStrategy::Error => {
                return Err(report);
            }
        }
    }

    let time = time.elapsed();
    info!("database migration complete ({:?})", time);
    Ok(database)
}

#[derive(Debug, Error)]
enum DatabaseMigrationError {
    #[error("database error: {0:?}")]
    DbErr(
        #[source]
        #[from]
        DbErr,
    ),
    #[error("database schema is not up to date it missing the following migrations: {missing_migrations}"
    )]
    NotMigrated { missing_migrations: String },
}

#[instrument(skip(database))]
async fn apply_migrations(
    database: &DatabaseConnection,
    migration_strategy: MigrationStrategy,
) -> Result<(), DatabaseMigrationError> {
    match migration_strategy {
        MigrationStrategy::Fresh => {
            warn!("database migration strategy is set to Fresh. Dropping and recreating the database...");
            Migrator::fresh(database)
                .await
                .map_err(DatabaseMigrationError::from)?;
        }
        MigrationStrategy::Apply => {
            Migrator::up(database, None)
                .await
                .map_err(DatabaseMigrationError::from)?;
        }
        MigrationStrategy::NoOp => {
            debug!("database migration strategy is set to NoOp. Skipping database migration.");
        }
        MigrationStrategy::Fail => {
            let pending = Migrator::get_pending_migrations(database)
                .await
                .map_err(DatabaseMigrationError::from)?;
            if !pending.is_empty() {
                let missing_migrations = pending
                    .iter()
                    .map(|m| m.name())
                    .collect::<Vec<_>>()
                    .join(", ");

                return Err(DatabaseMigrationError::NotMigrated { missing_migrations });
            }
        }
    }
    Ok(())
}

async fn init_ro_database(
    params: &DatabaseParams,
) -> error_stack::Result<Option<DatabaseConnection>, InitDatabaseError> {
    if let Some(database_ro_url) = &params.database_url_ro {
        let min_ro_connections = max(params.database_min_ro_connections, 1);
        let max_ro_connections = max(params.database_max_ro_connections, min_ro_connections);

        let ro_options = ConnectOptions::new(database_ro_url)
            .max_connections(max_ro_connections)
            .min_connections(min_ro_connections)
            .test_before_acquire(true)
            .sqlx_logging_level(LevelFilter::Debug)
            .sqlx_slow_statements_logging_settings(LevelFilter::Warn, Duration::from_secs(5 * 60))
            .to_owned();

        let database_ro = Database::connect(ro_options)
            .await
            .change_context(InitDatabaseError::ConnectToDatabase)?;

        Ok(Some(database_ro))
    } else {
        Ok(None)
    }
}

async fn init_redis(
    params: &DatabaseParams,
) -> error_stack::Result<redis::Client, InitDatabaseError> {
    info!("connecting to redis or a redis compatible server...");
    let time = Instant::now();
    let client = redis::Client::open(params.redis_url.clone())
        .change_context(InitDatabaseError::ConnectToDatabase)?;

    if !params.skip_connection_test {
        let mut connection = client
            .get_multiplexed_async_connection()
            .await
            .change_context(InitDatabaseError::ConnectToDatabase)?;
        let mut ping_cmd = Cmd::new();
        ping_cmd.arg("PING");
        connection
            .send_packed_command(&ping_cmd)
            .await
            .change_context(InitDatabaseError::ConnectToDatabase)?;
    }
    let time = time.elapsed();
    info!("redis connected ({:?})", time);

    Ok(client)
}
