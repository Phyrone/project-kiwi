use juniper::{graphql_object, ID};

#[derive(Clone, Debug)]
pub struct Profile {
    id: u64,
    name: String,
}

#[graphql_object]
impl Profile {
    async fn id(&self) -> ID {
        radix_fmt::radix_36(self.id).to_string().into()
    }


}
