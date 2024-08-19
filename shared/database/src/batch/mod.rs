use std::collections::HashMap;
use std::hash::Hash;
use std::ops::DerefMut;
use std::sync::Arc;
use std::time::Duration;
use dataloader::non_cached::Loader;
use error_stack::Report;
use moka::future::{Cache, CacheBuilder};
use moka::policy::EvictionPolicy;
use sea_orm::{DatabaseConnection, TransactionTrait};
use serde::{Deserialize, Serialize};
use thiserror::Error;

mod post_read;

pub type BatchResult<T> = Result<T, Arc<Report<BatchLoadError>>>;

#[derive(Debug, Clone, Error, Serialize, Deserialize)]
pub enum BatchLoadError {
    #[error("database operation failed")]
    DatabaseError,

    #[error("internal error")]
    InternalError,

    #[error("not found")]
    NotFound,
}

#[derive(Clone)]
pub struct EntityBatcher {
    account_read_cache: Cache<i64, BatchResult<crate::orm::account::Model>>,
    profile_read_cache: Cache<i64, BatchResult<crate::orm::profile::Model>>,
    post_read_cache: Cache<i64, BatchResult<crate::orm::post::Model>>,
    post_read_loader: Loader<i64, BatchResult<crate::orm::post::Model>, BatchContext>,
}

impl EntityBatcher {
    pub fn new(
        database: DatabaseConnection,
        database_ro: Option<DatabaseConnection>,
    ) -> Self {
        
        let post_read_cache = CacheBuilder::new(16 * 1024)
            .initial_capacity(128)
            .eviction_policy(EvictionPolicy::tiny_lfu())
            .time_to_live(Duration::from_secs(500))
            .time_to_idle(Duration::from_secs(30))
            .name("post_read_cache")
            .build();
        
        
        let context = BatchContext {
            database_ro: database_ro.unwrap_or_else(|| database.clone()),
            database,
            post_read_cache: post_read_cache.clone(),
        };
        let post_read_loader = Loader::new(context.clone());

        
        Self {
            post_read_cache,
            post_read_loader,
        }
    }

    pub async fn post_read(&self, id: i64) -> BatchResult<crate::orm::post::Model> {
        let cache = &self.post_read_cache;
        let loader = &self.post_read_loader;
        //TODO look ahead
        cache.get_with(id, loader.load(id)).await
    }
}

#[derive(Clone)]
pub struct BatchContext {
    pub database: DatabaseConnection,
    pub database_ro: DatabaseConnection,
    pub post_read_cache: Cache<i64, BatchResult<crate::orm::post::Model>>,
}

impl BatchContext {
    #[inline]
    pub fn err<K, V>(keys: &[K], err: error_stack::Report<BatchLoadError>) -> HashMap<K, BatchResult<V>>
    where
        K: Hash + Eq + Clone,
    {
        let err_ref = Arc::new(err);
        keys.iter()
            .map(|k| (k.clone(), Err(err_ref.clone())))
            .collect()
    }
}
