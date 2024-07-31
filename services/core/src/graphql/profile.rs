use async_graphql::ID;

#[derive(Default)]
pub struct ProfileQueryRoot {}

#[async_graphql::Object]
impl ProfileQueryRoot {
    async fn profile(&self) -> ProfileQuery {
        ProfileQuery {}
    }
}

pub struct ProfileQuery {}

#[async_graphql::Object(name = "Profile")]
impl ProfileQuery {
    async fn id(&self) -> ID {
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
