use std::collections::HashMap;
use std::sync::Arc;

use dataloader::BatchFn;
use error_stack::ResultExt;
use sea_orm::{
    AccessMode, ColumnTrait, DatabaseConnection, DatabaseTransaction, EntityTrait, IsolationLevel,
    QueryFilter, TransactionTrait,
};

use database::Profile;

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

        todo!()
    }
}

impl BatchFn<u64, database::orm::profile::Model> for GQLRequestContext {
    async fn load(&mut self, keys: &[u64]) -> HashMap<u64, database::orm::profile::Model> {
        let profiles = Profile::find()
            .filter(database::orm::profile::Column::Id.is_in(keys.iter().copied()))
            .all(&self.database_ro)
            .await;

        todo!()
    }
}
