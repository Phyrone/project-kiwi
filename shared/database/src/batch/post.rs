use std::collections::HashMap;
use std::process::Termination;
use std::sync::Arc;
use dataloader::BatchFn;
use dataloader::cached::Loader;
use error_stack::ResultExt;
use sea_orm::{ColumnTrait, LoaderTrait, DatabaseConnection, EntityTrait, QueryFilter, EntityOrSelect, IntoSimpleExpr};
use crate::batch::BatchLoadError;
use crate::orm::post::*;
use crate::Post;

type PostLoader<'db> = Loader<i64, Model, PostBatcher<'db>>;

pub struct PostBatcher<'db> {
    pub database_connection: &'db DatabaseConnection,
}

impl BatchFn<i64, error_stack::Result<Model,BatchLoadError>> for PostBatcher<'_> {
    async fn load(&mut self, keys: &[i64]) -> HashMap<i64, error_stack::Result<Model,BatchLoadError>> {
        let posts = Post::find()
            .filter(Column::Id.is_in(keys.iter().copied()))
            .all(self.database_connection)
            .await
            .map_err(|err| BatchLoadError::DatabaseError(Arc::new(err)));
        
        match posts {
            Ok(_) => {
                
            }
            Err(err) => {

                todo!()
            }
        }


        todo!()
    }
}

