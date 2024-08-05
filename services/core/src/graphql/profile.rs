use async_graphql::ID;

#[derive(Default)]
pub struct ProfileQueryRoot {}

#[async_graphql::Object]
impl ProfileQueryRoot {
    async fn profile(&self) -> Vec<ProfileQuery> {
        vec![]
    }
}

pub struct ProfileQuery {
    entity: database::orm::profile::Model,
}

#[async_graphql::Object(name = "Profile")]
impl ProfileQuery {
    async fn id(&self) -> ID {
        radix_fmt::radix_36(1);
        ID::from("id")
    }

    async fn name(&self) -> String {
        "name".to_string()
    }
}

pub struct ProfileMutation {}

#[async_graphql::Object(name = "ProfileMut")]
impl ProfileMutation {
    async fn set_name(&self, name: String) -> String {
        name
    }
}
