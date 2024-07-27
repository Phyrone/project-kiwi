use std::future::{Future, IntoFuture, poll_fn};
use std::net::SocketAddr;
use std::task::Poll;
use std::time::Duration;

use argon2::{Algorithm, Argon2, Params, ParamsBuilder, Version};
use axum::{Extension, Json, Router, ServiceExt};
use axum::extract::State;
use axum::http::{HeaderName, StatusCode};
use axum::response::{Html, IntoResponse, Redirect};
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
use tokio_util::sync::CancellationToken;
use tower_http::compression::{CompressionLayer, Predicate};
use tower_http::compression::predicate::{NotForContentType, SizeAbove};
use tower_http::CompressionLevel;
use tower_http::cors::CorsLayer;
use tower_http::request_id::PropagateRequestIdLayer;
use tower_http::timeout::TimeoutLayer;
use tracing::{info, instrument, warn};
use url::Url;
use webauthn_rs::{Webauthn, WebauthnBuilder};

use common::error_object;
use database::DatabaseInstance;
use web::rfc9457::{IntoProblemResultExt as ProblemResultExt, ProblemDescription};

use crate::web::graphql::graphql_endpoint;

pub mod graphql;

#[derive(Debug, Clone, Args)]
pub struct WebServerParams {
    #[clap(short, long, default_value = "0.0.0.0:3000", env = "BIND")]
    pub binds: Vec<SocketAddr>,
}

error_object!(RunWebServerError, "Failed to run web server");

#[instrument(level = "debug", skip(params, database))]
pub async fn run_web_server(
    params: WebServerParams,
    database: DatabaseInstance,
    shutdown_token: CancellationToken,
) -> error_stack::Result<(), RunWebServerError> {
    let router = Router::new()
        .fallback(get(not_found))
        .merge(graphql_endpoint(database.clone()))
        //.layer(TimeoutLayer::new(Duration::from_secs(10)))
        .into_make_service_with_connect_info::<SocketAddr>();

    let mut all_binds = JoinSet::new();
    for bind in params.binds {
        let listener = TcpListener::bind(bind)
            .await
            .change_context(RunWebServerError)?;
        info!("Listening on http://{}", listener.local_addr().unwrap());
        let router = router.clone();
        let shutdown_token = shutdown_token.clone();
        let server = axum::serve(listener, router)
            .with_graceful_shutdown(shutdown_token.cancelled_owned());
        all_binds.spawn(server.into_future());
    }

    if let Some(result) = all_binds.join_next().await {
        if !shutdown_token.is_cancelled() {
            warn!("Server stopped but no shutdown signal appears to be received, sending one now");
            shutdown_token.cancel();
        }
        result
            .change_context(RunWebServerError)?
            .change_context(RunWebServerError)?;
    }

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "status")]
pub enum HealthCheckResponse {
    Ok,
    DatabaseUnavailable,
}

const NOT_FOUND_HTML: &str = include_str!("404.html");

async fn not_found() -> Html<&'static str> {
    Html(NOT_FOUND_HTML)
}
