use async_graphql::{Context, Json};
use redis::AsyncCommands;
use webauthn_rs::prelude::{PublicKeyCredential, RequestChallengeResponse};

#[derive(Default)]
pub struct AuthRoot;

#[async_graphql::Object]
impl AuthRoot {
    async fn me(&self) -> Option<String> {
        None
    }
}

#[derive(Default)]
pub struct AuthRootMut;

#[async_graphql::Object]
impl AuthRootMut {
    async fn webautn_login_start(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<Json<RequestChallengeResponse>> {
        todo!()
    }

    async fn webauth_login_finish(
        &self,
        ctx: &Context<'_>,
        credential: Json<PublicKeyCredential>,
    ) -> async_graphql::Result<Option<String>> {
        todo!()
    }
}
