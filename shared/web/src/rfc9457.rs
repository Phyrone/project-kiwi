use aide::gen::GenContext;
use aide::openapi::{MediaType, Operation, SchemaObject};
use aide::OperationOutput;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use indexmap::IndexMap;
use schemars::JsonSchema;
use sea_orm::DbErr;
use serde::{Deserialize, Serialize};

const PROBLEM_JSON_MIME_TYPE: &str = "application/problem+json";
pub const PROBLEM_TYPE_BASE: &str = "https://comming.soon/problems";

pub type ProblemResult<T, E = ()> = Result<T, ProblemDescription<E>>;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ProblemDescription<T = ()> {
    pub status: u16,
    #[serde(rename = "type")]
    pub problem_type: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,

    #[serde(flatten)]
    pub data: T,
}

impl Default for ProblemDescription {
    fn default() -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            problem_type: format!("{}/internal-server-error", PROBLEM_TYPE_BASE),
            title: "Internal Server Error".to_string(),
            detail: None,
            instance: None,
            data: (),
        }
    }
}

impl<T> IntoResponse for ProblemDescription<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let body = serde_json::to_string(&self).expect("Failed to serialize problem description");

        let response = Response::builder()
            .status(self.status)
            .header("Content-Type", PROBLEM_JSON_MIME_TYPE)
            .body(body)
            .expect("Failed to build response");
        response.into_response()
    }
}

impl<T> OperationOutput for ProblemDescription<T>
where
    T: Serialize + JsonSchema + Send + Sync + 'static,
{
    type Inner = Self;

    fn operation_response(
        ctx: &mut GenContext,
        _operation: &mut Operation,
    ) -> Option<aide::openapi::Response> {
        let mut schema = ctx
            .schema
            .subschema_for::<ProblemDescription>()
            .into_object();

        Some(aide::openapi::Response {
            description: schema.metadata().description.clone().unwrap_or_default(),
            links: IndexMap::from_iter([]),
            content: IndexMap::from_iter([(
                PROBLEM_JSON_MIME_TYPE.to_string(),
                MediaType {
                    schema: Some(SchemaObject {
                        json_schema: schema.into(),
                        example: None,
                        external_docs: None,
                    }),
                    ..Default::default()
                },
            )]),
            ..Default::default()
        })
    }
}

pub trait IntoProblemResultExt<T, E> {
    fn into_problem(self) -> ProblemResult<T, E>;
}

impl<T, E, D> IntoProblemResultExt<T, D> for Result<T, E>
where
    E: Into<ProblemDescription<D>>,
{
    fn into_problem(self) -> ProblemResult<T, D> {
        self.map_err(Into::into)
    }
}

impl From<DbErr> for ProblemDescription {
    fn from(err: DbErr) -> Self {
        ProblemDescription {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            problem_type: format!("{PROBLEM_TYPE_BASE}/database-error"),
            title: "Database Error".to_string(),
            detail: Some(err.to_string()),
            instance: None,
            data: (),
        }
    }
}
