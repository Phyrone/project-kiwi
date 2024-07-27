use juniper::{FieldError, FieldResult, graphql_object, graphql_value};
use webauthn_rs::prelude::*;

use crate::graphql::context::GraphqlContext;

#[derive(Clone, Debug)]
pub struct KiwiAuthentication {}

impl KiwiAuthentication {
    pub fn new() -> Self {
        Self {}
    }
}

#[graphql_object]
#[graphql(context = GraphqlContext)]
impl KiwiAuthentication {
    /// Invalidate all sessions for the current user.
    async fn logout_all(
        &self, context: &GraphqlContext,
        user_id: String,
    ) -> bool {
        false
    }


    async fn magic_link(
        &self,
        context: &GraphqlContext,
        email: String,
    ) -> bool {
        true
    }

    async fn authn_challenge(
        &self,
        context: &GraphqlContext,
        email: String,
    ) -> FieldResult<bool> {
        let url = Url::parse("").unwrap();
        let authn = WebauthnBuilder::new("", &url)
            .unwrap()
            .build()
            .unwrap();


        Ok(true)
    }

    async fn magic_link_login(
        &self,
        context: &GraphqlContext,
        email: String,
        token: String,
    ) -> bool {
        true
    }
}