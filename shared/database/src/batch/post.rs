use std::collections::HashMap;
use std::sync::Arc;

use dataloader::cached::Loader;
use dataloader::BatchFn;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::batch::BatchLoadError;
use crate::orm::post::*;
use crate::Post;

type PostLoader<'db> = Loader<i64, Model, PostBatcher<'db>>;

pub struct PostBatcher<'db> {
    pub database_connection: &'db DatabaseConnection,
}

impl BatchFn<i64, error_stack::Result<Model, BatchLoadError>> for PostBatcher<'_> {
    async fn load(
        &mut self,
        keys: &[i64],
    ) -> HashMap<i64, error_stack::Result<Model, BatchLoadError>> {
        let posts = Post::find()
            .filter(Column::Id.is_in(keys.iter().copied()))
            .all(self.database_connection)
            .await
            .map_err(|err| BatchLoadError::DatabaseError(Arc::new(err)));

        match posts {
            Ok(_) => {}
            Err(err) => {
                todo!()
            }
        }

        todo!()
    }
}
