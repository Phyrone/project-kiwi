use axum::Json;
use juniper::{graphql_object, ID, Object, Value};
use crate::graphql::context::GraphqlContext;
use crate::graphql::profile::Profile;
use crate::graphql::publication::PublicationValue;

pub struct Comment {
    id: i64,
}

#[graphql_object]
#[graphql(impl = PublicationValue, context = GraphqlContext)]
impl Comment {
    fn id(&self) -> ID {
        radix_fmt::radix_36(self.id).to_string().into()
    }

    fn author(&self) -> Profile {
        todo!()
    }

    fn metadata(&self) -> String {
        todo!()
    }
    

    fn content(&self) -> String {
        todo!()
    }
}
