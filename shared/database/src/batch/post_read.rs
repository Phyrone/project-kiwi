use crate::batch::{BatchContext, BatchLoadError, BatchResult};
use dataloader::BatchFn;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use std::collections::HashMap;
use std::sync::Arc;
use error_stack::{FutureExt, Report};
use crate::post::{Column, Model};
use crate::prelude::Post;


impl BatchFn<i64, BatchResult<Model>> for BatchContext {
    async fn load(&mut self, ids: &[i64]) -> HashMap<i64, BatchResult<Model>> {
        let mut res = HashMap::with_capacity(ids.len());

        let posts = Post::find()
            .filter(Column::Id.is_in(ids.iter().copied()))
            .all(&self.database_ro)
            .change_context(BatchLoadError::DatabaseError)
            .await;

        match posts {
            Err(report) => return BatchContext::err(ids, report),
            Ok(posts) => {
                for post in posts {
                    res.insert(post.id, Ok(post));
                }
                let missing = ids.iter().filter(|id| !res.contains_key(id)).collect::<Vec<_>>();
                if !missing.is_empty() {
                    let not_found_error = Arc::new(Report::new(BatchLoadError::NotFound));
                    for key in missing {
                        res.insert(*key, Err(not_found_error.clone()));
                    }
                }
            }
        }
        res
    }
}