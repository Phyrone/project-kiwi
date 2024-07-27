use std::sync::Arc;
use std::time::Duration;
use axum::{Extension, Json, Router};
use axum::extract::{Query, WebSocketUpgrade};
use axum::http::Method;
use axum::response::IntoResponse;
use axum::routing::{get, MethodFilter, on};
use axum_auth::AuthBearer;
use axum_either::AxumEither;
use juniper::DefaultScalarValue;
use juniper_axum::{graphiql, playground};
use juniper_axum::extract::JuniperRequest;
use juniper_axum::response::JuniperResponse;
use juniper_axum::subscriptions::serve_ws;
use juniper_graphql_ws::{ConnectionConfig, Schema};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use tower_http::compression::{CompressionLayer, Predicate};
use tower_http::compression::predicate::{NotForContentType, SizeAbove};
use tower_http::CompressionLevel;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer, MaxAge};
use tower_http::follow_redirect::FollowRedirectLayer;
use tower_http::request_id::PropagateRequestIdLayer;
use tracing::instrument;

use common::Error;
use database::DatabaseInstance;

use crate::graphql::{
    GraphQLSchema, GraphQlTransport, KiwiService, KiwiServiceMutable, KiwiServiceSubscription,
};
use crate::graphql::context::GraphqlContext;

const GRAPHQL_PATH: &str = "/graphql";
const GRAPHIQL_PATH: &str = "/graphiql";
const PLAYGROUND_PATH: &str = "/playground";
const SUBSCRIPTIONS_PATH: &str = "/subscriptions";

#[derive(Debug, Clone)]
pub struct WebRequestContext {
    db: DatabaseConnection,
    db_ro: Option<DatabaseConnection>,
    schema: Arc<GraphQLSchema>,
}

pub fn graphql_endpoint(
    database_connection: DatabaseInstance
) -> Router<()> {
    let DatabaseInstance {
        db: database_connection,
        db_ro: database_connection_ro,
    } = database_connection;
    let schema = GraphQLSchema::new(
        KiwiService {},
        KiwiServiceMutable {},
        KiwiServiceSubscription {},
    );
    let ctx = WebRequestContext {
        db: database_connection,
        db_ro: database_connection_ro,
        schema: Arc::new(schema),
    };


    Router::new()
        .route(SUBSCRIPTIONS_PATH, get(graphql_handle_ws))
        .route(GRAPHQL_PATH, on(MethodFilter::GET.or(MethodFilter::POST), graphql_handle_http))
        .route(
            GRAPHIQL_PATH,
            get(graphiql(GRAPHQL_PATH, SUBSCRIPTIONS_PATH)),
        )
        .route(
            PLAYGROUND_PATH,
            get(playground(GRAPHQL_PATH, SUBSCRIPTIONS_PATH)),
        )
        .layer(Extension(ctx))
        .layer(CompressionLayer::new())
        .layer(
            CompressionLayer::new()
                .br(true)
                .zstd(true)
                .gzip(true)
                .deflate(true)
                .quality(CompressionLevel::Default)
                .compress_when(
                    SizeAbove::new(512)
                        .and(NotForContentType::GRPC)
                        .and(NotForContentType::IMAGES)
                        .and(NotForContentType::SSE),
                ),
        )
        .layer(CorsLayer::new()
            .allow_methods(AllowMethods::list(vec![Method::GET, Method::POST]))
            .allow_origin(AllowOrigin::mirror_request())
            .allow_headers(AllowHeaders::mirror_request())
            .allow_credentials(false)
            .max_age(MaxAge::exact(Duration::from_secs(60)))
        ).layer(PropagateRequestIdLayer::x_request_id())
}

#[cfg_attr(text, axum::debug_handler)]
#[instrument(level = "debug")]
pub async fn graphql_handle_http(
    token: Option<AuthBearer>,
    Extension(ctx): Extension<WebRequestContext>,
    JuniperRequest(req): JuniperRequest<DefaultScalarValue>,
) -> JuniperResponse<DefaultScalarValue> {
    let token = token.map(|t| t.0);

    let context = GraphqlContext::new(GraphQlTransport::Http, token, ctx.db, ctx.db_ro, None).await;
    let response = req.execute(ctx.schema.root_node(), &context).await;
    JuniperResponse(response)
}


#[derive(Debug, Clone, Deserialize)]
pub struct GraphQlSubscriptionQueryStrings {
    pub token: Option<String>,
}

#[cfg_attr(text, axum::debug_handler)]
#[instrument(level = "debug")]
pub async fn graphql_handle_hybrid(
    request: axum_either::AxumEither<WebSocketUpgrade, Json<()>>,
    token: Option<AuthBearer>,
    Query(query): Query<GraphQlSubscriptionQueryStrings>,
    Extension(ctx): Extension<WebRequestContext>,
) -> Box<dyn IntoResponse> {
    let token = token.map(|t| t.0).or(query.token);
    let context = GraphqlContext::new(GraphQlTransport::Websocket, token, ctx.db, ctx.db_ro, None).await;

    let config = ConnectionConfig::new(context);
    match request {
        AxumEither::Left(ws) => {
            Box::new(ws.protocols(["graphql-transport-ws", "graphql-ws"])
                .max_frame_size(1024)
                .max_message_size(1024)
                .on_upgrade(|socket| serve_ws(socket, ctx.schema, config)))
        }
        AxumEither::Right(tower_http) => {
            todo!()
        }
    }
}

#[cfg_attr(text, axum::debug_handler)]
#[instrument(level = "debug")]
pub async fn graphql_handle_ws(
    ws: WebSocketUpgrade,
    token: Option<AuthBearer>,
    Query(query): Query<GraphQlSubscriptionQueryStrings>,
    Extension(ctx): Extension<WebRequestContext>,
) -> impl IntoResponse {
    let token = token.map(|t| t.0).or(query.token);
    let context = GraphqlContext::new(GraphQlTransport::Websocket, token, ctx.db, ctx.db_ro, None).await;

    let config = ConnectionConfig::new(context);
    ws.protocols(["graphql-transport-ws", "graphql-ws"])
        .max_frame_size(1024)
        .max_message_size(1024)
        .on_upgrade(|socket| serve_ws(socket, ctx.schema, config))
}

#[derive(Debug, Error)]
#[error("Failed to initialize subscription context")]
pub struct InitSubscriptionContextError;