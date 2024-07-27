use std::fmt::Display;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;

use context::GraphqlContext;
use futures::Stream;
use juniper::{
    graphql_object, graphql_subscription, Context, FieldError, GraphQLEnum, RootNode, ID,
};
use sea_orm::{ConnectionTrait, TransactionTrait};

use crate::graphql::profile::{Profile, ProfileMut};

pub mod comment;
pub mod context;
pub mod guild;
pub mod media;
pub mod post;
pub mod profile;
pub mod publication;
mod auth;

pub type GraphQLSchema =
    RootNode<'static, KiwiService, KiwiServiceMutable, KiwiServiceSubscription>;

/// The transport used to communicate with the server.
#[derive(Clone, Debug, GraphQLEnum)]
#[graphql(name = "Transport")]
pub enum GraphQlTransport {
    /// The transport is using HTTP.
    Http,
    /// The transport is using Websockets.
    Websocket,
}

#[derive(Clone, Debug)]
pub struct KiwiService {}

#[derive(Clone, Debug)]
pub struct KiwiServiceMutable {}

#[graphql_object]
#[graphql(context = GraphqlContext)]
impl KiwiService {
    async fn number(#[graphql(context)] ctx: &GraphqlContext) -> i32 {
        42
    }

    /// Returns the transport used to communicate with the server.
    /// the value only represetns the transport used between the actual core and a peer.
    /// f.e. when some node inbetween maps the transport to another one, the value will represent the one used between the core and the node.
    /// Its not realy useful and more for debugging purposes.
    fn transport(&self, context: &GraphqlContext) -> GraphQlTransport {
        context.transport.clone()
    }

    /// Returns the baerer token used to authenticate the request. Only for testing purposes. Not existent in release builds.
    #[cfg(debug_assertions)]
    #[graphql(
        name = "token",
        deprecated = "unsafe only for testing, use 'me' instead!"
    )]
    async fn token(&self, context: &GraphqlContext) -> Option<String> {
        let token = &context.token;

        token.clone()
    }

    /// Returns the current user profile if authenticated. Otherwise null.
    /// This is the prefered way to get the current user profile.
    async fn me() -> Option<Profile> {
        None
    }
    async fn profile(id: Option<ID>, name: Option<String>) -> Option<Profile> {
        None
    }

    async fn profiles(
        #[graphql(desc = "The number of profiles to return")] limit: Option<i32>,
        #[graphql(desc = "The offset to start from")] offset: Option<i32>,
    ) -> Vec<Profile> {
        vec![]
    }
}

#[graphql_object]
#[graphql(context = GraphqlContext)]
impl KiwiServiceMutable {
    
    async fn auth() -> auth::KiwiAuthentication {
        auth::KiwiAuthentication::new()
    }
    
    async fn edit_profile(profile_id: ID, domain: Option<String>) -> Option<ProfileMut> {
        None
    }
}

#[derive(Clone, Debug)]
pub struct KiwiServiceSubscription {}

type StringStream = Pin<Box<dyn Stream<Item = Result<String, FieldError>> + Send>>;

#[graphql_subscription]
#[graphql(context = GraphqlContext)]
impl KiwiServiceSubscription {
    async fn hello_world() -> StringStream {
        let stream = futures::stream::iter([Ok(String::from("Hello")), Ok(String::from("World!"))]);
        Box::pin(stream)
    }
    async fn hello_world2() -> StringStream {
        let stream = futures::stream::iter([Ok(String::from("Hello")), Ok(String::from("World!"))]);
        Box::pin(stream)
    }
}
