use juniper::{graphql_interface, ID};

use crate::graphql::comment::Comment;
use crate::graphql::context::GraphqlContext;
use crate::graphql::media::MediaImage;
use crate::graphql::post::Post;
use crate::graphql::profile::Profile;

#[allow(unused)]
#[graphql_interface]
#[graphql(for = [Post, Comment], context = GraphqlContext)]
pub trait Publication {
    fn id(&self) -> ID;

    async fn author(&self) -> Profile;
}
