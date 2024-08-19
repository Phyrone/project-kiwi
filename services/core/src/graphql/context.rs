use std::collections::HashMap;
use std::sync::Arc;

use dataloader::BatchFn;
use error_stack::ResultExt;
use sea_orm::{
    AccessMode, ColumnTrait, DatabaseConnection, DatabaseTransaction, EntityTrait, IsolationLevel,
    QueryFilter, TransactionTrait,
};
use database::prelude::Profile;
use crate::web::graphql::WebContext;

#[derive(thiserror::Error, Debug)]
pub enum CreateGQLRequestContextError {
    #[error("cannot begin a read transaction")]
    CreateReadTransactionError,
}

#[derive(Clone)]
pub struct GQLRequestContext {
    database: DatabaseConnection,
    database_ro: DatabaseConnection,
    txn_ro: Arc<DatabaseTransaction>,
}

impl GQLRequestContext {
    pub async fn new(
        web_context: WebContext,
    ) -> error_stack::Result<Self, CreateGQLRequestContextError> {
        let database_ro = web_context
            .db
            .db_ro
            .unwrap_or_else(|| web_context.db.db.clone());
        let database = web_context.db.db;

        let txn_ro = database_ro
            .begin_with_config(
                Some(IsolationLevel::RepeatableRead),
                Some(AccessMode::ReadOnly),
            )
            .await
            .change_context(CreateGQLRequestContextError::CreateReadTransactionError)?;

        Ok(Self {
            database,
            database_ro,
            txn_ro: Arc::new(txn_ro),
        })
    }
}