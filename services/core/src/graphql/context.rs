use std::sync::Arc;

use juniper::Context;
use sea_orm::{AccessMode, DatabaseConnection, DatabaseTransaction, DbErr, IsolationLevel, TransactionTrait};
use tokio::sync::{RwLock, RwLockReadGuard};

use crate::graphql::GraphQlTransport;

#[derive(Clone)]
pub struct GraphqlContext {
    pub transport: GraphQlTransport,
    pub token: Option<String>,
    pub db: DatabaseConnection,
    pub db_ro: Option<DatabaseConnection>,

    pub txn_ro: Arc<RwLock<Option<DatabaseTransaction>>>,
    pub txn_rw: Arc<RwLock<Option<DatabaseTransaction>>>,
}

impl GraphqlContext {
    #[inline]
    pub async fn new(
        transport: GraphQlTransport,
        token: Option<String>,
        database: DatabaseConnection,
        database_ro: Option<DatabaseConnection>,
        //When a tranaction was already started f.e. to validate session data, it can be passed here.
        txn_ro: Option<DatabaseTransaction>,
    ) -> Self {
        Self {
            transport,
            token,
            db: database,
            db_ro: database_ro,
            txn_ro: Arc::new(RwLock::new(txn_ro)),
            txn_rw: Arc::new(RwLock::new(None)),
        }
    }

    async fn read_only_transaction(&self) -> error_stack::Result<RwLockReadGuard<DatabaseTransaction>, DbErr> {
        let maybe_txn = self.txn_ro.read().await;
        if maybe_txn.is_some() {
            return Ok(RwLockReadGuard::map(maybe_txn, |txn| txn.as_ref().unwrap()));
        } else {
            //When no transaction is started yet, start one.
            let mut txn_init = self.txn_ro.write().await;
            let db = self.db_ro.as_ref().unwrap_or(&self.db);
            //Double check since between 2 locks, another thread could have initialized the transaction.
            // Which would mean the other transaction would be dropped -> no lost but inconsistent data
            if txn_init.is_none(){
                let txn = self.db.begin_with_config(
                    Some(IsolationLevel::RepeatableRead),
                    Some(AccessMode::ReadWrite),
                ).await?;
                txn_init.replace(txn);
            }
            let txn_guard = txn_init.downgrade();
            Ok(RwLockReadGuard::map(txn_guard, |txn| txn.as_ref().unwrap()))
        }
    }
    async fn read_write_transaction(&self) -> error_stack::Result<RwLockReadGuard<DatabaseTransaction>, DbErr> {
        let maybe_txn = self.txn_rw.read().await;
        if maybe_txn.is_some() {
            return Ok(RwLockReadGuard::map(maybe_txn, |txn| txn.as_ref().unwrap()));
        } else {
            let mut txn_init = self.txn_rw.write().await;
            //Double check since between 2 locks, another thread could have initialized the transaction.
            // Which would mean the other transaction would be dropped -> lost update
            if txn_init.is_none(){
                let txn = self.db.begin_with_config(
                    Some(IsolationLevel::RepeatableRead),
                    Some(AccessMode::ReadWrite),
                ).await?;
                txn_init.replace(txn);
            }
            let txn_guard = txn_init.downgrade();
            Ok(RwLockReadGuard::map(txn_guard, |txn| txn.as_ref().unwrap()))
        }
    }
    
}

impl Context for GraphqlContext {}
