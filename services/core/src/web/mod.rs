use std::future::{Future, IntoFuture, poll_fn};
use std::net::SocketAddr;
use std::task::Poll;
use std::time::Duration;

use argon2::{Algorithm, Argon2, Params, ParamsBuilder, Version};
use axum::{Extension, Json, Router, ServiceExt};
use axum::extract::State;
use axum::http::{HeaderName, Method, StatusCode};
use axum::response::{Html, IntoResponse, Redirect};
use axum::routing::get;
use clap::Args;
use error_stack::ResultExt as ErrorStackResultExt;
use futures_lite::FutureExt;
use lazy_static::lazy_static;
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
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer, MaxAge};
use tower_http::request_id::PropagateRequestIdLayer;
use tower_http::timeout::TimeoutLayer;
use tracing::{info, instrument, warn};
use url::Url;
use webauthn_rs::{Webauthn, WebauthnBuilder};

use common::error_object;
use database::DatabaseInstance;
use web::auth::WEBAUTHN_CHALLENGE_TIMEOUT;
use web::rfc9457::{IntoProblemResultExt as ProblemResultExt, ProblemDescription};

use crate::web::graphql::graphql_endpoint;

pub mod graphql;
mod auth;

#[derive(Debug, Clone, Args)]
pub struct WebServerParams {
    #[clap(short, long, default_value = "0.0.0.0:3000", env = "BIND")]
    pub binds: Vec<SocketAddr>,

    #[clap(long, env = "WEBAUTHN_RP_ID")]
    pub webauthn_rp_id: String,
    
    #[clap(long, env = "WEBAUTHN_RP_ORIGIN")]
    pub webauthn_rp_origin: Url,

    #[clap(long, default_value = "Project Kiwi", env = "WEBAUTHN_RP_NAME")]
    pub webauthn_rp_name: String,
}


#[derive(Debug, thiserror::Error)]
pub enum RunWebServerError{
    #[error("webauthn setup failed")]
    SetupWebauthn,
    
    #[error("failed to open socket")]
    OpenSocket,
    #[error("failed to start/run server")]
    ServeAxum
}

#[instrument(level = "debug", skip(params, database))]
pub async fn run_web_server(
    params: WebServerParams,
    database: DatabaseInstance,
    shutdown_token: CancellationToken,
) -> error_stack::Result<(), RunWebServerError> {
   let origin = Url::parse(params.webauthn_rp_origin.as_ref())
       .change_context(RunWebServerError::SetupWebauthn)?;
    
    let webauthn = WebauthnBuilder::new(
        &params.webauthn_rp_id,
        &origin,
    ).change_context(RunWebServerError::SetupWebauthn)?
        .timeout(WEBAUTHN_CHALLENGE_TIMEOUT)
        .build()
        .change_context(RunWebServerError::SetupWebauthn)?;
    
    let router = Router::new()
        .fallback(get(not_found))
        .merge(graphql_endpoint(database.clone(),webauthn))
        //.layer(TimeoutLayer::new(Duration::from_secs(10)))
        .layer(CompressionLayer::new())
        .layer(
            CompressionLayer::new()
                .br(true)
                .zstd(true)
                .gzip(true)
                .deflate(true)
                .quality(CompressionLevel::Default)
                .compress_when(
                    SizeAbove::new(1024)
                        .and(NotForContentType::GRPC)
                        .and(NotForContentType::IMAGES)
                        .and(NotForContentType::SSE),
                ),
        )
        .layer(
            CorsLayer::new()
                .allow_methods(AllowMethods::list(vec![Method::GET, Method::POST]))
                .allow_origin(AllowOrigin::mirror_request())
                .allow_headers(AllowHeaders::mirror_request())
                .allow_credentials(false)
                .max_age(MaxAge::exact(Duration::from_secs(60))),
        )
        .layer(PropagateRequestIdLayer::x_request_id())
        .into_make_service_with_connect_info::<SocketAddr>();

    let mut all_binds = JoinSet::new();
    for bind in params.binds {
        let listener = TcpListener::bind(bind)
            .await
            .change_context(RunWebServerError::OpenSocket)?;
        info!("Listening on http://{}", listener.local_addr().unwrap());
        let router = router.clone();
        let shutdown_token = shutdown_token.clone();
        let server =
            axum::serve(listener, router).with_graceful_shutdown(shutdown_token.cancelled_owned());
        all_binds.spawn(server.into_future());
    }

    if let Some(result) = all_binds.join_next().await {
        if !shutdown_token.is_cancelled() {
            warn!("Server stopped but no shutdown signal appears to be received, sending one now");
            shutdown_token.cancel();
        }
        result
            .change_context(RunWebServerError::ServeAxum)?
            .change_context(RunWebServerError::ServeAxum)?;
    }

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "status")]
pub enum HealthCheckResponse {
    Ok,
    DatabaseUnavailable,
}

async fn root() -> Redirect {
    Redirect::to("/graphiql")
}

lazy_static! {
    static ref MINIFIED_404_HTML: String = minify::html::minify(include_str!("404.html"));
}

async fn not_found() -> Html<&'static str> {
    Html(&MINIFIED_404_HTML)
}
