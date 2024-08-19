use async_graphql::extensions::ApolloTracing;
use async_graphql::http::{
    playground_source, GraphQLPlaygroundConfig, GraphiQLSource, ALL_WEBSOCKET_PROTOCOLS,
};
use async_graphql::{async_trait, Data, SDLExportOptions};
use async_graphql_axum::{GraphQLBatchRequest, GraphQLProtocol, GraphQLResponse, GraphQLWebSocket};
use axum::extract::{FromRequest, Query, Request, WebSocketUpgrade};
use axum::http::Method;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, on, MethodFilter};
use axum::{Extension, RequestExt, Router};
use axum_auth::AuthBearer;
use error_stack::ResultExt;
use http::StatusCode;
use lazy_static::lazy_static;
use serde::Deserialize;
use tracing::instrument;
use webauthn_rs::Webauthn;

use database::DatabaseInstance;

use crate::graphql::context::GQLRequestContext;
use crate::graphql::{
    AuthToken, GraphQLSchema, GraphQlTransport, KiwiQuery, KiwiQueryMut, KiwiSubscription,
};

const GRAPHQL_PATH: &str = "/graphql";
const GRAPHIQL_PATH: &str = "/graphiql";
const SCHEMA_PATH: &str = "/schema";

pub const MAX_QUERY_COMPLEXITY: usize = 1024;

fn build_schema(with_tracing: bool) -> GraphQLSchema {
    let builder = GraphQLSchema::build(
        KiwiQuery::default(),
        KiwiQueryMut::default(),
        KiwiSubscription::default(),
    )
        .limit_complexity(MAX_QUERY_COMPLEXITY)
        .with_sorted_fields()
        .with_sorted_enums();
    let builder = if with_tracing {
        builder.extension(ApolloTracing)
    } else {
        builder
    };
    builder.finish()
}
lazy_static! {
    static ref SCHEMA: GraphQLSchema = build_schema(false);
    static ref SCHEMA_WITH_TRACING: GraphQLSchema = build_schema(true);
}

#[derive(Clone)]
pub struct WebContext {
    pub webauthn: Webauthn,
    pub db: DatabaseInstance,
}

pub fn graphql_endpoint(database_instance: DatabaseInstance, webauthn: Webauthn) -> Router<()> {
    let ctx = WebContext {
        webauthn,
        db: database_instance,
    };

    Router::new()
        .route(
            GRAPHQL_PATH,
            on(MethodFilter::GET.or(MethodFilter::POST), graphql_handle),
        )
        .route(GRAPHIQL_PATH, get(graphiql))
        .route(SCHEMA_PATH, get(graphql_schema_export))
        .layer(Extension(ctx))
}

enum MultiGraphQlExtractor {
    Http(GraphQLBatchRequest),
    WebSocket((WebSocketUpgrade, GraphQLProtocol)),
}
impl MultiGraphQlExtractor {
    fn transport(&self) -> GraphQlTransport {
        match self {
            MultiGraphQlExtractor::Http(_) => GraphQlTransport::Http,
            MultiGraphQlExtractor::WebSocket(_) => GraphQlTransport::WebSocket,
        }
    }
}

#[async_trait::async_trait]
impl<S> FromRequest<S> for MultiGraphQlExtractor
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(mut req: Request, state: &S) -> Result<Self, Self::Rejection> {
        if req.method() == Method::GET {
            let maybe_upgrade = req
                .extract_parts_with_state::<WebSocketUpgrade, S>(state)
                .await;
            let protocol = req
                .extract_parts_with_state::<GraphQLProtocol, S>(state)
                .await;
            if let Ok(upgrade) = maybe_upgrade {
                if let Ok(protocol) = protocol {
                    return Ok(MultiGraphQlExtractor::WebSocket((upgrade, protocol)));
                }
            }
            let no_query = req
                .uri()
                .query()
                .map(|q| q.trim().is_empty())
                .unwrap_or(true);
            if no_query {
                return Err(Html(playground_source(
                    GraphQLPlaygroundConfig::new(GRAPHQL_PATH)
                        .subscription_endpoint(GRAPHQL_PATH)
                        .title("Playground for Project Kiwi"),
                ))
                    .into_response());
            }
        }
        GraphQLBatchRequest::from_request(req, state)
            .await
            .map(MultiGraphQlExtractor::Http)
            .map_err(IntoResponse::into_response)
    }
}

enum GraphQlMultiResponse {
    Http(GraphQLResponse),
    WebSocket(Response),
}
impl IntoResponse for GraphQlMultiResponse {
    fn into_response(self) -> Response {
        match self {
            GraphQlMultiResponse::Http(response) => response.into_response(),
            GraphQlMultiResponse::WebSocket(upgrade) => upgrade,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GraphQlQueryStrings {
    pub token: Option<String>,
}

#[derive(thiserror::Error, Debug)]
pub enum GraphQLQueryError {
    #[error("cannot create context")]
    CreateContextError,
}

#[instrument(level = "debug", skip(ctx, req))]
pub async fn graphql_handle(
    token: Option<AuthBearer>,
    Query(query): Query<GraphQlQueryStrings>,
    Extension(ctx): Extension<WebContext>,
    req: MultiGraphQlExtractor,
) -> Result<GraphQlMultiResponse, impl IntoResponse> {
    let token = token.map(|t| t.0).or(query.token);
    let transport = req.transport();
    let context = GQLRequestContext::new(ctx).await;
    match context {
        Ok(context) => match req {
            MultiGraphQlExtractor::Http(request) => {
                let transport = GraphQlTransport::Http;

                let req = request
                    .into_inner()
                    .data(transport)
                    .data(AuthToken(token))
                    .data(context);

                let response = SCHEMA.execute_batch(req).await.into();
                Ok(GraphQlMultiResponse::Http(response))
            }
            MultiGraphQlExtractor::WebSocket((ws, protocol)) => {
                let mut data = Data::default();
                data.insert(transport);
                data.insert(AuthToken(token));
                data.insert(context);
                let upgrade = ws
                    .protocols(ALL_WEBSOCKET_PROTOCOLS)
                    .on_upgrade(move |socket| {
                        GraphQLWebSocket::new(socket, SCHEMA.clone(), protocol)
                            .with_data(data)
                            .serve()
                    });
                Ok(GraphQlMultiResponse::WebSocket(upgrade))
            }
        },
        Err(report) => {
            tracing::error!("Error creating context: \n{:?}", report);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn graphql_schema_export() -> String {
    SCHEMA.sdl_with_options(
        SDLExportOptions::new()
            .include_specified_by()
            .prefer_single_line_descriptions(),
    )
}

async fn graphiql() -> Html<String> {
    Html(
        GraphiQLSource::build()
            .endpoint(GRAPHQL_PATH)
            .subscription_endpoint(GRAPHQL_PATH)
            .finish(),
    )
}
