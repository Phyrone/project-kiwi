use aide::axum::routing::{delete, get, get_with, post, put, put_with};
use aide::axum::{ApiRouter, IntoApiResponse};
use aide::transform::TransformOperation;
use axum::extract::{Path, Query};
use axum::Json;
use axum_auth::AuthBearer;
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

use web::rfc9457::ProblemDescription;

use crate::web::WebServerState;

pub fn post_routes() -> ApiRouter<WebServerState> {
    ApiRouter::new()
        .api_route("/posts/:domain/:id", get(get_post_handler))
        .api_route("/posts/:domain/:id", post(update_post_handler))
        .api_route("/posts/:domain/:id", delete(delete_post_handler))
        .api_route("/posts", put_with(create_post_handler, create_post_docs))
        .api_route(
            "/posts",
            get_with(get_explore_posts_handler, get_explore_posts_docs),
        )
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreatePostRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<i64>,
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreatePostResponse {
    pub post_id: i64,
}

fn create_post_docs(t: TransformOperation) -> TransformOperation {
    t.response_with::<200, Json<CreatePostResponse>, _>(|a| {
        a.example(CreatePostResponse { post_id: 42 })
    })
    .response_with::<400, ProblemDescription, _>(|a| {
        a.example(ProblemDescription {
            status: 400,
            problem_type: "https://example.com/problems/bad-request".to_string(),
            title: "Bad Request".to_string(),
            detail: None,
            instance: None,
            data: (),
        })
    })
}

async fn create_post_handler(
    Json(body): Json<CreatePostRequest>,
) -> Result<Json<CreatePostResponse>, ProblemDescription> {
    Ok(Json(CreatePostResponse { post_id: 1 }))
}

fn get_explore_posts_docs(t: TransformOperation) -> TransformOperation {
    t
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct ExplorePostsQueryParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    //TODO filter queries
}

async fn get_explore_posts_handler(
    //auth: AuthBearer,
    Query(query): Query<ExplorePostsQueryParams>,
) -> impl IntoApiResponse {
    "trending posts here"
}

async fn get_post_handler(Path((domain, id)): Path<(String, i64)>) -> impl IntoApiResponse {
    format!("get post with domain: {} and id: {} here", domain, id)
}

async fn update_post_handler(Path((domain, id)): Path<(String, i64)>) -> impl IntoApiResponse {}

async fn delete_post_handler(Path((domain, id)): Path<(String, i64)>) -> impl IntoApiResponse {}
