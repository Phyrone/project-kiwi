use async_graphql::{Context, DataContext, Json, SimpleObject};

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE;
use jwt::ToBase64;
use nanoid::nanoid;
use redis::{AsyncCommands, ExistenceCheck, SetExpiry, SetOptions};
use webauthn_rs::prelude::{DiscoverableAuthentication, PublicKeyCredential, RequestChallengeResponse};
use webauthn_rs::Webauthn;

use database::DatabaseInstance;
use web::auth::{WEBAUTHN_CHALLENGE_PREFIX, WEBAUTHN_CHALLENGE_TIMEOUT_SECONDS};
use crate::graphql::context::GQLRequestContext;
use crate::web::graphql::{MAX_QUERY_COMPLEXITY};

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
    
    #[graphql(complexity=MAX_QUERY_COMPLEXITY)]
    async fn webautn_login_start(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<Json<RequestChallengeResponse>> {
        let webauthn = &ctx.data_unchecked::<GQLRequestContext>().webauthn;
        let redis_client = &ctx.data_unchecked::<GQLRequestContext>().db.redis;
        let (challenge, auth_state) = webauthn.start_discoverable_authentication()?;
        let challenge_data = URL_SAFE.encode(&challenge.public_key.challenge);

        let key = format!("{}/{}", WEBAUTHN_CHALLENGE_PREFIX, &challenge_data);
        let data = rmp_serde::to_vec_named(&auth_state)?;
        let set_options = SetOptions::default()
            .with_expiration(SetExpiry::EX(WEBAUTHN_CHALLENGE_TIMEOUT_SECONDS))
            .conditional_set(ExistenceCheck::NX);
        let mut connection = redis_client.get_multiplexed_tokio_connection().await?;
        let _: bool = connection.set_options(key, data, set_options).await?;
        
        Ok(Json(challenge))
    }

    #[graphql(complexity=MAX_QUERY_COMPLEXITY)]
    async fn webauth_login_finish(
        &self,
        ctx: &Context<'_>,
        challenge: String,
        credential: Json<PublicKeyCredential>,
    ) -> async_graphql::Result<Option<String>> {
        
        let webauthn = ctx.data_unchecked::<Webauthn>();
        let redis_client = &ctx.data_unchecked::<DatabaseInstance>().redis;
        let key = format!("{}/{}", WEBAUTHN_CHALLENGE_PREFIX, &challenge);
        let mut connection = redis_client.get_multiplexed_tokio_connection().await?;
        let data: Vec<u8> = connection.get_del(key).await?;
        let auth_state = rmp_serde::from_slice::<DiscoverableAuthentication>(&data)?;
        

        let (user_id, key_id) = webauthn.identify_discoverable_authentication(&credential)?;
     
        let auth = webauthn.finish_discoverable_authentication(&credential, auth_state,  &[])?;
        
        
        todo!()
    }
}