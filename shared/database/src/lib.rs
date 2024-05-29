use std::time::Instant;

use clap::{Args, ValueEnum};
use colored::Colorize;
use sea_orm::{Database, DatabaseConnection};
use tracing::{debug, info, instrument, warn};

use common::error_stack::ResultExt;
use common::Error;
use migration::{Migrator, MigratorTrait};

pub mod orm;
mod redis;

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

    //TODO more documentation
    /// Database URL.
    #[clap(long = "database-url", env = "DATABASE_URL")]
    database_url: String,
}

#[derive(Debug, Default, Clone, ValueEnum)]
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
    ///never do anything with the database schema. Its to the administrator to keep the database schema up to date.
    #[value(aliases = ["none", "skip", "ignore"])]
    NoOp,
}

#[derive(Debug, Error)]
pub enum InitDatabaseError {
    #[error("an error occured while connecting to the database")]
    ConnectToDatabase,
    #[error("an error occured while migrating the database")]
    MigrateDatabase,
}

#[instrument]
pub async fn init_database(
    params: &DatabaseParams,
) -> error_stack::Result<DatabaseConnection, InitDatabaseError> {
    debug!("database url: {}", params.database_url.bright_blue());

    info!("connecting to the database...");
    let time = Instant::now();
    let database = Database::connect(&params.database_url)
        .await
        .change_context(InitDatabaseError::ConnectToDatabase)?;
    let time = time.elapsed();
    info!("database connected ({:?})", time);

    let time = Instant::now();
    match params.migration_strategy {
        MigrationStrategy::Fresh => {
            warn!("database migration strategy is set to Fresh. Dropping and recreating the database...");
            Migrator::fresh(&database)
                .await
                .change_context(InitDatabaseError::MigrateDatabase)?;
        }
        MigrationStrategy::Apply => {
            Migrator::up(&database, None)
                .await
                .change_context(InitDatabaseError::MigrateDatabase)?;
        }
        MigrationStrategy::NoOp => {
            debug!("database migration strategy is set to NoOp. Skipping database migration.");
        }
    }
    let time = time.elapsed();
    info!("database migration done ({:?})", time);

    Ok(database)
}
