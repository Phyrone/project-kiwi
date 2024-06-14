use juniper::{EmptySubscription, graphql_object, ID, RootNode};

pub mod profile;

#[derive(Clone, Debug)]
pub struct KiwiServiceQuery {}

#[derive(Clone, Debug)]
pub struct KiwiServiceMutation {}

#[graphql_object]
impl KiwiServiceQuery {
    async fn number() -> i32 {
        42
    }

    async fn profile(profile_id: ID) -> profile::Profile {

        todo!()
    }


}

#[graphql_object]
impl KiwiServiceMutation {
    async fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}


pub type GraphQLSchema = RootNode<'static, KiwiServiceQuery, KiwiServiceMutation, EmptySubscription<()>>;