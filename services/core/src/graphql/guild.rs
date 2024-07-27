use crate::graphql::context::GraphqlContext;
use juniper::{graphql_object, ID};

pub struct Guild {
    id: i64,
}

#[graphql_object]
#[graphql(context = GraphqlContext)]
impl Guild {
    fn id(&self) -> ID {
        radix_fmt::radix_36(self.id).to_string().into()
    }
}
