use aide::axum::{ApiRouter, IntoApiResponse};
use aide::axum::routing::{get_with, post, post_with};
use aide::transform::TransformOperation;
use argon2::{Algorithm, Argon2, Params, ParamsBuilder, PasswordHash, PasswordVerifier, Version};
use axum::{Json, Router};
use axum::extract::{Request, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum_auth::AuthBearer;
use schemars::JsonSchema;
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, ModelTrait, QueryFilter, TransactionTrait};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use common::error_object;
use web::rfc9457::{IntoProblemResultExt, PROBLEM_TYPE_BASE, ProblemDescription, ProblemResult};

use crate::web::WebServerState;

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
        .await.into_problem()?;

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
    response
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AuthStatusFailureResponse {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AuthStatusSuccessResponse {
    pub success: bool,
    pub user_id: i64,
}

fn auth_status_docs(t: TransformOperation) -> TransformOperation {
    t.description("Request the status of the current login session")
        .security_requirement("bearerAuth")
}

async fn auth_status(
    State(server): State<WebServerState>,
    //AuthBearer(token): AuthBearer,
    headers: HeaderMap,
) -> ProblemResult<Json<AuthStatusSuccessResponse>, AuthStatusFailureResponse> {
    let auth = headers.get("Authorization");
    if let Some(auth) = auth.and_then(|auth| auth.to_str().ok()) {
        Ok(Json(AuthStatusSuccessResponse {
            success: true,
            user_id: 1536267501962947106,
        }))
    } else {
        Err(ProblemDescription {
            status: StatusCode::UNAUTHORIZED.as_u16(),
            problem_type: format!("{}/not-logged-in", PROBLEM_TYPE_BASE),
            title: "Unauthorized".to_string(),
            detail: Some("There is no active login session.".to_string()),
            instance: None,
            data: AuthStatusFailureResponse { success: false },
        })
    }
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
    State(server): State<WebServerState>,
    Json(body): Json<LogoutAllRequest>,
) -> Result<Json<LogoutAllResponse>, ProblemDescription> {
    let WebServerState { database, argon2 } = server;
    database.begin();
    todo!()
}
