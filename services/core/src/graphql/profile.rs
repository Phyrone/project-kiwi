use std::collections::HashMap;

use juniper::{graphql_object, ID};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use database::orm::profile::Column as ProfileColumn;
use database::orm::profile::Entity as ProfileEntity;
use database::orm::profile::Model as ProfileModel;

use crate::graphql::context::GraphqlContext;
use crate::graphql::media::MediaImage;
use crate::graphql::post::{Post, PostMut};

struct ProfileLoader {}

impl dataloader::BatchFn<ID, ProfileModel> for ProfileLoader {
    async fn load(&mut self, keys: &[ID]) -> HashMap<ID, ProfileModel> {
        todo!()
    }
}

#[derive(Clone, Debug)]
pub struct Profile {
    id: i64,
}

#[graphql_object]
#[graphql(context = GraphqlContext)]
impl Profile {
    async fn id(&self) -> ID {
        radix_fmt::radix_36(self.id).to_string().into()
    }

    async fn avatar(&self) -> Option<MediaImage> {
        None
    }

    async fn name(&self) -> String {
        todo!()
    }

    async fn post(&self, id: ID) -> Option<Post> {
        todo!()
    }

    async fn posts(
        &self,
        #[graphql(desc = "The number of posts to return")] limit: Option<i32>,
        #[graphql(desc = "The offset to start from")] offset: Option<i32>,
    ) -> Vec<Post> {
        vec![]
    }
}

pub struct ProfileMut {
    id: i64,
}

#[graphql_object]
#[graphql(context = GraphqlContext)]
impl ProfileMut {
    async fn id(&self) -> ID {
        radix_fmt::radix_36(self.id).to_string().into()
    }

    async fn name(&self) -> String {
        todo!()
    }

    async fn post(&self, id: ID) -> Option<PostMut> {
        todo!()
    }

    async fn set_name(&self, name: String) -> String {
        todo!()
    }
}
