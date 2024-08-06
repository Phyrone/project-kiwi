use std::collections::HashMap;

use dataloader::BatchFn;
use sea_orm::EntityTrait;

use crate::batch::{BatchContext, BatchResult};
use crate::post::Model;

impl BatchFn<i64, BatchResult<Model>> for BatchContext {
    async fn load(&mut self, keys: &[i64]) -> HashMap<i64, BatchResult<Model>> {
        todo!()
    }
}