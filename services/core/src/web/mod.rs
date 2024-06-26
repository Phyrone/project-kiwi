use std::future::{poll_fn, Future, IntoFuture};
use std::net::SocketAddr;
use std::task::Poll;
use std::time::Duration;

use aide::axum::routing::get as api_get;
use aide::axum::routing::post as api_post;
use aide::axum::{ApiRouter, IntoApiResponse};
use aide::openapi::{Info, OpenApi};
use argon2::{Algorithm, Argon2, Params, ParamsBuilder, Version};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use axum::routing::get;
use axum::{Extension, Json, Router, ServiceExt};
use clap::Args;
use error_stack::ResultExt as ErrorStackResultExt;
use futures_lite::FutureExt;
use schemars::JsonSchema;
use sea_orm::{ConnectionTrait, DatabaseConnection};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::Database;
use tokio::net::TcpListener;
use tower_http::compression::predicate::NotForContentType;
use tower_http::compression::{CompressionLayer, DefaultPredicate, Predicate};
use tower_http::timeout::TimeoutLayer;
use tower_http::CompressionLevel;
use tracing::{info, instrument};
use utoipa_swagger_ui::SwaggerUi;

use common::error_object;
use web::rfc9457::{IntoProblemResultExt as ProblemResultExt, ProblemDescription};

mod auth;
mod post;

#[derive(Debug, Clone, Args)]
pub struct WebServerParams {
    #[clap(short, long, default_value = "0.0.0.0:3000", env = "BIND")]
    pub binds: Vec<SocketAddr>,
}

error_object!(RunWebServerError, "Failed to run web server");

#[derive(Clone)]
pub struct WebServerState {
    database: DatabaseConnection,
    argon2: Argon2<'static>,
}

impl WebServerState {
    fn new(database: DatabaseConnection) -> Self {
        const PARAMS_MULTIPLIER: u32 = 8;
        const OUTPUT_BASE_LENGTH: usize = 32;
        let argon2 = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            ParamsBuilder::new()
                .t_cost(Params::DEFAULT_T_COST * PARAMS_MULTIPLIER)
                .p_cost(Params::DEFAULT_P_COST * PARAMS_MULTIPLIER)
                .m_cost(Params::DEFAULT_M_COST * PARAMS_MULTIPLIER)
                .output_len(OUTPUT_BASE_LENGTH * PARAMS_MULTIPLIER as usize)
                .build()
                .expect("Failed to build argon2 params"),
        );
        Self { database, argon2 }
    }
}

#[instrument]
pub async fn run_web_server(
    params: WebServerParams,
    database: DatabaseConnection,
) -> error_stack::Result<(), RunWebServerError> {
    let mut open_api = OpenApi {
        info: Info {
            title: "Kiwi Core API".to_string(),
            description: Some("api spec for kiwi core api".to_string()),
            ..Info::default()
        },
        ..OpenApi::default()
    };

    let api_router = ApiRouter::new()
        .route("/", api_get(root))
        .api_route("/health", api_get(healthcheck))
        .merge(post::post_routes())
        .merge(auth::auth_routes())
        .finish_api(&mut open_api);

    let open_api_spec = serde_json::to_value(&open_api).change_context(RunWebServerError)?;

    let api_router = api_router.layer(Extension(open_api));

    let router = Router::new()
        .merge(api_router)
        .with_state(WebServerState::new(database))
        .merge(SwaggerUi::new("/swagger").external_url_unchecked("/openapi.json", open_api_spec))
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .layer(
            CompressionLayer::new()
                .br(true)
                .zstd(true)
                .gzip(true)
                .deflate(true)
                .quality(CompressionLevel::Best)
                .compress_when(DefaultPredicate::default()),
        )
        .into_make_service_with_connect_info::<SocketAddr>();

    let mut servers = Vec::new();
    for bind in params.binds {
        let listener = TcpListener::bind(bind)
            .await
            .change_context(RunWebServerError)?;
        info!("Listening on http://{}", listener.local_addr().unwrap());
        let router = router.clone();
        let server = axum::serve(listener, router).into_future();
        servers.push(server);
    }

    poll_fn(|cx| {
        for server in &mut servers {
            let poll = server.poll(cx);
            if poll.is_ready() {
                return Poll::Ready(());
            }
        }
        Poll::Pending
    })
    .await;

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "status")]
pub enum HealthCheckResponse {
    Ok,
    DatabaseUnavailable,
}

async fn healthcheck(
    State(database): State<WebServerState>,
) -> Result<(StatusCode, Json<HealthCheckResponse>), ProblemDescription> {
    let WebServerState { database, .. } = database;
    database.ping().await.into_problem()?;

    Ok((StatusCode::OK, Json(HealthCheckResponse::Ok)))
}

async fn root() -> impl IntoApiResponse {
    Redirect::permanent("/swagger/")
}
