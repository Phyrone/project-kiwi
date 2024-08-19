use dataloader::non_cached::Loader;
use error_stack::Report;
use moka::future::{Cache, CacheBuilder};
use moka::policy::EvictionPolicy;
use sea_orm::{DatabaseConnection, TransactionTrait};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::DerefMut;
use std::sync::Arc;
use std::time::Duration;
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
    post_read_loader: Loader<i64, BatchResult<crate::orm::post::Model>, BatchContext>,
}

#[derive(Clone)]
pub struct EntityCache {
    account_read_cache: Cache<i64, BatchResult<crate::orm::account::Model>>,
    profile_read_cache: Cache<i64, BatchResult<crate::orm::profile::Model>>,
    post_read_cache: Cache<i64, BatchResult<crate::orm::post::Model>>,
}

#[derive(Clone)]
pub struct BatchContext {
    pub database: DatabaseConnection,
    pub database_ro: DatabaseConnection,
    pub cache: EntityCache,
}

impl EntityBatcher {
    pub fn new(database: DatabaseConnection, database_ro: Option<DatabaseConnection>) -> Self {
        let cache = EntityCache::default();
        let context = BatchContext {
            database_ro: database_ro.unwrap_or_else(|| database.clone()),
            database,
            cache,
        };
        let post_read_loader = Loader::new(context.clone());

        Self { post_read_loader }
    }

    pub async fn post_read(&self, id: i64) -> BatchResult<crate::orm::post::Model> {
        let cache = &self.post_read_cache;
        let loader = &self.post_read_loader;
        //TODO look ahead
        cache.get_with(id, loader.load(id)).await
    }
}

impl Default for EntityCache {
    fn default() -> Self {
        let post_read_cache = CacheBuilder::new(16 * 1024)
            .initial_capacity(128)
            .eviction_policy(EvictionPolicy::tiny_lfu())
            .time_to_live(Duration::from_secs(500))
            .time_to_idle(Duration::from_secs(30))
            .name("post_read_cache")
            .build();
        let account_read_cache = CacheBuilder::new(16 * 1024)
            .initial_capacity(128)
            .eviction_policy(EvictionPolicy::tiny_lfu())
            .time_to_live(Duration::from_secs(500))
            .time_to_idle(Duration::from_secs(30))
            .name("account_read_cache")
            .build();
        let profile_read_cache = CacheBuilder::new(16 * 1024)
            .initial_capacity(128)
            .eviction_policy(EvictionPolicy::tiny_lfu())
            .time_to_live(Duration::from_secs(500))
            .time_to_idle(Duration::from_secs(30))
            .name("profile_read_cache")
            .build();

        Self {
            post_read_cache,
            account_read_cache,
            profile_read_cache,
        }
    }
}

impl BatchContext {
    #[inline]
    pub fn err<K, V>(
        keys: &[K],
        err: error_stack::Report<BatchLoadError>,
    ) -> HashMap<K, BatchResult<V>>
    where
        K: Hash + Eq + Clone,
    {
        let err_ref = Arc::new(err);
        keys.iter()
            .map(|k| (k.clone(), Err(err_ref.clone())))
            .collect()
    }
}
