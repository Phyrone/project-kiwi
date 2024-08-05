use std::sync::Arc;

use sea_orm::{DatabaseConnection, DatabaseTransaction, DbErr};
use thiserror::Error;
use tokio::sync::{Mutex, MutexGuard, RwLock, RwLockReadGuard};

mod post;

#[derive(Debug, Clone, Error)]
pub enum BatchLoadError {
    #[error("database operation failed {0}")]
    DatabaseError(Arc<sea_orm::error::DbErr>),
}

pub struct BatchContext {
    database_connection_rw: DatabaseConnection,
    database_connection_ro: Option<DatabaseConnection>,
    txn_rw: Arc<Mutex<Option<DatabaseTransaction>>>,
    txn_ro: Arc<RwLock<Option<DatabaseTransaction>>>,
}

impl BatchContext {
    pub async fn txn_rw(&mut self) -> Result<MutexGuard<DatabaseTransaction>, DbErr> {
        todo!()
    }
    pub async fn txn_ro(&mut self) -> Result<RwLockReadGuard<DatabaseTransaction>, DbErr> {
        todo!()
    }
}
