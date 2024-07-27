use std::sync::Arc;
use thiserror::Error;

mod post;


#[derive(Debug, Clone, Error)]
pub enum BatchLoadError {
    #[error("database operation failed {0}")]
    DatabaseError(Arc<sea_orm::error::DbErr>),
}