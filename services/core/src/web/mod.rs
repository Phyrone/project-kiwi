use std::future::{Future, IntoFuture, poll_fn};
use std::net::SocketAddr;
use std::task::Poll;
use std::time::Duration;

use argon2::{Algorithm, Argon2, Params, ParamsBuilder, Version};
use axum::{Extension, Json, Router, ServiceExt};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use axum::routing::get;
use clap::Args;
use error_stack::ResultExt as ErrorStackResultExt;
use futures_lite::FutureExt;
use schemars::JsonSchema;
use sea_orm::{ConnectionTrait, DatabaseConnection};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::Database;
use tokio::net::TcpListener;
use tokio::task::JoinSet;
use tower_http::compression::{CompressionLayer, DefaultPredicate, Predicate};
use tower_http::compression::predicate::{NotForContentType, SizeAbove};
use tower_http::CompressionLevel;
use tower_http::timeout::TimeoutLayer;
use tracing::{info, instrument};
use url::Url;
use webauthn_rs::{Webauthn, WebauthnBuilder};

use common::error_object;
use web::rfc9457::{IntoProblemResultExt as ProblemResultExt, ProblemDescription};

use crate::web::graphql::graphql_endpoint;

mod graphql;

#[derive(Debug, Clone, Args)]
pub struct WebServerParams {
    #[clap(short, long, default_value = "0.0.0.0:3000", env = "BIND")]
    pub binds: Vec<SocketAddr>,
}

error_object!(RunWebServerError, "Failed to run web server");


#[instrument(level = "debug", skip(params, database))]
pub async fn run_web_server(
    params: WebServerParams,
    database: DatabaseConnection,
) -> error_stack::Result<(), RunWebServerError> {
    let router = Router::new()
        .merge(graphql_endpoint(database.clone()))
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .layer(
            CompressionLayer::new()
                .br(true)
                .zstd(true)
                .gzip(true)
                .deflate(true)
                .quality(CompressionLevel::Default)
                .compress_when(
                    SizeAbove::new(128)
                        .and(NotForContentType::GRPC)
                        .and(NotForContentType::IMAGES)
                        .and(NotForContentType::SSE),
                ),
        )
        .into_make_service_with_connect_info::<SocketAddr>();

    let mut all_binds = JoinSet::new();
    for bind in params.binds {
        let listener = TcpListener::bind(bind)
            .await
            .change_context(RunWebServerError)?;
        info!("Listening on http://{}", listener.local_addr().unwrap());
        let router = router.clone();
        let server = axum::serve(listener, router).into_future();
        all_binds.spawn(server);
    }

    if let Some(result) = all_binds.join_next().await {
        result
            .change_context(RunWebServerError)?
            .change_context(RunWebServerError)?;
        all_binds.shutdown().await
    }
    
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "status")]
pub enum HealthCheckResponse {
    Ok,
    DatabaseUnavailable,
}


async fn root() -> impl IntoResponse {
    Redirect::permanent("/graphiql")
}
