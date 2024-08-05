use std::time::Duration;

use async_graphql::futures_util::Stream;
use async_graphql::{
    async_stream, scalar, Context, DataContext, Enum, MergedObject, MergedSubscription, Object,
    ScalarType, Schema, ID,
};
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

use crate::graphql::auth::{AuthRoot, AuthRootMut};

mod auth;
pub mod context;
mod profile;

#[derive(Debug, Clone)]
pub struct AuthToken(pub Option<String>);

/// A simple rate limiter which prohibits calling a function more than once per query call.
/// f.e. you cannot create more than one webauthn challenge per query call.
pub struct QueryCallLimiter {}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct Unit;
scalar!(Unit, "Unit");

#[derive(Clone)]
pub struct RequestData {
    pub transport: GraphQlTransport,
    pub token: Option<String>,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum GraphQlTransport {
    Http,
    WebSocket,
}

pub(crate) type GraphQLSchema = Schema<KiwiQuery, KiwiQueryMut, KiwiSubscription>;

#[derive(MergedObject, Default)]
#[graphql(name = "Query")]
pub struct KiwiQuery(KiwiBase, AuthRoot, profile::ProfileQueryRoot);

#[derive(MergedObject, Default)]
#[graphql(name = "Mutation")]
pub struct KiwiQueryMut(AuthRootMut);

#[derive(MergedSubscription, Default)]
#[graphql(name = "Subscription")]
pub struct KiwiSubscription(SubscriptionsTest);

#[derive(Default)]
pub struct KiwiBase;

#[async_graphql::Object]
impl KiwiBase {
    async fn transport<'a>(&self, ctx: &Context<'a>) -> &'a GraphQlTransport {
        ctx.data_unchecked::<GraphQlTransport>()
    }
}

#[derive(Default)]
pub struct SubscriptionsTest;

#[async_graphql::Subscription]
impl SubscriptionsTest {
    pub async fn test(
        &self,
        #[graphql(
            desc = "The step size between each number",
            default = 1,
            validator(minimum = 1)
        )]
        step_size: i64,
    ) -> impl Stream<Item = TestObject> {
        async_stream::stream! {
            for i in 0..1024_i64 {
                sleep(Duration::from_secs(1)).await;
                let test_object = TestObject { id: (i*step_size) };
                yield  test_object;
            }
        }
    }
}
pub struct TestObject {
    pub id: i64,
}

#[Object]
impl TestObject {
    async fn id(&self) -> ID {
        ID::from(base36::encode(&self.id.to_be_bytes()))
    }
    async fn id_i64(&self) -> i64 {
        self.id
    }
}
