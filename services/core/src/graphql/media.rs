use juniper::{graphql_interface, graphql_object, GraphQLEnum, ID};

use crate::graphql::context::GraphqlContext;

#[allow(unused)]
#[graphql_interface]
#[graphql(for = [MediaImage], context = GraphqlContext)]
pub trait Media {
    fn id(&self) -> ID;

    fn url(&self) -> String;

    fn size(&self, human_readable: Option<bool>) -> String;
}

#[derive(Debug, Clone)]
pub struct MediaImage {
    id: i64,
}

#[graphql_object]
#[graphql(name = "Image", impl = MediaValue, context = GraphqlContext)]
impl MediaImage {
    fn id(&self) -> ID {
        radix_fmt::radix_36(self.id).to_string().into()
    }

    async fn url(&self) -> String {
        todo!()
    }

    async fn format(&self) -> String {
        todo!()
    }

    fn width(&self) -> i32 {
        todo!()
    }

    fn height(&self) -> i32 {
        todo!()
    }

    fn size(&self, human_readable: Option<bool>) -> String {
        todo!()
    }

    fn variants(&self) -> Vec<MediaImageVariant> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct MediaImageVariant {
    id: i64,
}

#[graphql_object]
#[graphql(name = "ImageVariant", context = GraphqlContext)]
impl MediaImageVariant {
    async fn url(&self) -> String {
        todo!()
    }

    fn format(&self) -> String {
        todo!()
    }

    fn width(&self) -> i32 {
        todo!()
    }
    fn height(&self) -> i32 {
        todo!()
    }

    fn size(&self, human_readable: Option<bool>) -> String {
        todo!()
    }
}
