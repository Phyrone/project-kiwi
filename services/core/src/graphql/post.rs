use juniper::{graphql_object, ID};

use crate::graphql::comment::Comment;
use crate::graphql::context::GraphqlContext;
use crate::graphql::media::MediaValue;
use crate::graphql::profile::Profile;
use crate::graphql::publication::{Publication, PublicationValue};

#[derive(Debug, Clone)]
pub struct Post {
    id: i64,
}

#[graphql_object]
#[graphql(impl = PublicationValue, context = GraphqlContext)]
impl Post {
    fn id(&self) -> ID {
        radix_fmt::radix_36(self.id).to_string().into()
    }

    fn author(&self) -> Profile {
        todo!()
    }

    fn title(&self) -> String {
        todo!()
    }

    fn content(&self) -> String {
        todo!()
    }

    fn comments(
        &self,
        #[graphql(desc = "The number of comments to return")] limit: Option<i32>,
        #[graphql(desc = "The offset to start from")] offset: Option<i32>,
    ) -> Vec<Comment> {
        vec![]
    }

    fn attachments(&self) -> Vec<MediaValue> {
        todo!()
    }
}

pub struct PostMut {
    id: i64,
}

#[graphql_object]
#[graphql(context = GraphqlContext)]
impl PostMut {
    async fn id(&self) -> ID {
        radix_fmt::radix_36(self.id).to_string().into()
    }

    async fn author(&self) -> Profile {
        todo!()
    }

    async fn title(&self) -> String {
        todo!()
    }

    async fn content(&self) -> String {
        todo!()
    }

    async fn comments(
        &self,
        #[graphql(desc = "The number of comments to return")] limit: Option<i32>,
        #[graphql(desc = "The offset to start from")] offset: Option<i32>,
    ) -> Vec<Comment> {
        vec![]
    }
}
