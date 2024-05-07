use aide::axum::routing::{get_with, post, post_with};
use aide::axum::{ApiRouter, IntoApiResponse};
use aide::transform::TransformOperation;
use argon2::{Algorithm, Argon2, Params, ParamsBuilder, PasswordVerifier, Version};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Json, Router};
use axum_auth::AuthBearer;
use schemars::JsonSchema;
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, ModelTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::web::WebServerState;
use common::error_object;
use web::rfc9457::{ErrorExt, ProblemDescription, ProblemResult, PROBLEM_TYPE_BASE};

error_object!(CreateAuthRoutesError, "Failed to register auth routes");
pub fn auth_routes() -> ApiRouter<WebServerState> {
    ApiRouter::new()
        .api_route("/auth/login", post_with(request_login, request_login_docs))
        .api_route("/auth/status", get_with(auth_status, auth_status_docs))
        .api_route("/auth/logout-all", post(logout_all_handler))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LoginRequest {
    pub user: String,
    #[serde(flatten)]
    pub with: LoginRequestWith,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "with", rename_all = "snake_case")]
pub enum LoginRequestWith {
    Email,
    Passkey { passkey: Box<Value> },
    Password { password: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AuthProblem {
    success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LoginSuccess {
    /// Whether the login was successful
    pub success: bool,

    /// The user snowflake id of the user was successfully authenticated.
    /// Javascript devs keep in mind this id likely exceeds the max safe integer value (2^53 - 1)
    pub user_id: i64,
    /// A bearer token to be used for future requests to authenticate as the user.
    pub token: String,
}

fn request_login_docs(t: TransformOperation) -> TransformOperation {
    t.hidden(false)
        .description("Request a login token with the given credentials")
        .response_with::<200, Json<LoginSuccess>, _>(|res| {
            res.example(LoginSuccess {
                success: true,
                user_id: 1536267501962947106,
                token: "BASE64 encoded secure token".to_string(),
            })
        })
        .response_with::<401, ProblemDescription<AuthProblem>, _>(|res| {
            res.example(ProblemDescription {
                status: StatusCode::UNAUTHORIZED.as_u16(),
                problem_type: format!("{}/unauthorized", PROBLEM_TYPE_BASE),
                title: "Unauthorized".to_string(),
                detail: Some("The email or password provided is incorrect.".to_string()),
                instance: None,
                data: AuthProblem { success: false },
            })
        })
}

async fn request_login(
    State(server): State<WebServerState>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoApiResponse {
    let WebServerState { database, argon2 } = server;
    let user_model = database::orm::account::Entity::find()
        .filter(database::orm::account::Column::Email.eq(payload.user.to_lowercase()))
        .one(&database)
        .await;

    let response: Response = match user_model {
        Err(err) => err.into_problem().into_response(),
        Ok(user_model) => {
            if let Some(user) = user_model {
                match payload.with {
                    LoginRequestWith::Email => {
                        todo!()
                    }
                    LoginRequestWith::Passkey { passkey } => {
                        todo!()
                    }
                    LoginRequestWith::Password { password } => {
                        todo!()
                    }
                }
            } else {
                ProblemDescription {
                    status: StatusCode::UNAUTHORIZED.as_u16(),
                    problem_type: format!("{}/unauthorized", PROBLEM_TYPE_BASE),
                    title: "Unauthorized".to_string(),
                    detail: Some("The email or password provided is incorrect.".to_string()),
                    instance: None,
                    data: AuthProblem { success: false },
                }
                .into_response()
            }
        }
    };
    response
}

fn auth_status_docs(t: TransformOperation) -> TransformOperation {
    t.description("Request the status of the current login session")
}

async fn auth_status(
    State(server): State<WebServerState>,
    //AuthBearer(token): AuthBearer,
) -> impl IntoApiResponse {
    todo!()
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LogoutAllRequest {
    #[serde(default)]
    renew_current_token: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LogoutAllResponse {
    new_token: Option<String>,
}

/// Logs out all sessions for the current user and invalidates all tokens
async fn logout_all_handler(
    Json(body): Json<LogoutAllRequest>,
) -> Result<Json<LogoutAllResponse>, ProblemDescription> {
    todo!()
}
