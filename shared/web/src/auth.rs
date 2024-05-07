use std::hash::Hash;

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD as B64;
use digest::Mac;
use sea_orm::{ConnectionTrait, EntityTrait};
use serde::{Deserialize, Serialize};

use common::{error_object, error_stack};
use common::error_stack::ResultExt;

type HmacSha256 = hmac::Hmac<sha3::Sha3_256>;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct UserToken {
    pub payload: UserTokenPayload,
    pub signature: [u8; 32],
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct UserTokenPayload {
    pub user_id: i64,
    pub valid_since: u64,
    pub valid_until: Option<u64>,
    //TODO add more fields like region, etc.
}

error_object!(TokenEncodeError, "cloud not encode token");
error_object!(TokenDecodeError, "cloud not decode token");

impl UserToken {
    pub fn encode(&self) -> error_stack::Result<String, TokenEncodeError> {
        let binary = bincode::serialize(self)
            .change_context(TokenEncodeError)
            .attach_printable("encode into binary")?;
        Ok(B64.encode(binary))
    }

    pub fn decode(token: &str) -> error_stack::Result<Self, TokenDecodeError> {
        let binary = B64
            .decode(token.as_bytes())
            .change_context(TokenDecodeError)
            .attach_printable("decode from base64")?;
        let decoded = bincode::deserialize(&binary)
            .change_context(TokenDecodeError)
            .attach_printable("decode binary token token")?;

        Ok(decoded)
    }

    pub fn valid_signature(&self, secret: &[u8], namespace: &str) -> bool {
        let expected = self.payload.create_signature(secret, namespace);
        self.signature == expected
    }
}

impl UserTokenPayload {
    pub fn valid_time(&self) -> bool {
        let now = chrono::Utc::now().timestamp() as u64;
        self.valid_since <= now && self.valid_until.unwrap_or(u64::MAX) >= now
    }

    pub fn create_signature(&self, secret: &[u8], namespace: &str) -> [u8; 32] {
        let mut hasher = HmacSha256::new_from_slice(secret).expect("should never fail");
        hasher.update(namespace.as_bytes());
        hasher.update(&self.user_id.to_be_bytes());
        hasher.update(&self.valid_since.to_be_bytes());
        if let Some(valid_until) = self.valid_until {
            hasher.update(&[0x01; 1]);
            hasher.update(&valid_until.to_be_bytes());
        } else {
            hasher.update(&[0xFF; 9])
        }
        let result = hasher.finalize().into_bytes();
        let result: [u8; 32] = result
            .to_vec()
            .try_into()
            .expect("hash value has the wrong length this should not happen");

        result
    }
    pub fn singed(&self, secret: &[u8], namespace: &str) -> UserToken {
        let signature = self.create_signature(secret, namespace);
        UserToken {
            payload: self.clone(),
            signature,
        }
    }
}

pub async fn check_user_token<C>(token: &str, database: &C, namespace: &str) -> Option<UserTokenPayload>
    where
        C: ConnectionTrait,
{
    let token = UserToken::decode(token).ok()?;
    if !token.payload.valid_time() {
        return None;
    }

    let user_entity = database::orm::account::Entity::find_by_id(token.payload.user_id)
        .one(database)
        .await
        .ok()??;

    if token.valid_signature(&user_entity.session_secret, namespace) {
        Some(token.payload)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    #[test]
    pub fn test_signature() {
        const NAMESPACE: &str = "test";
        let payload = super::UserTokenPayload {
            user_id: 188287775496339456,
            valid_since: chrono::Utc::now().timestamp() as u64,
            valid_until: None,
        };
        let secret = b"secret";
        let token = payload.singed(secret, NAMESPACE);
        let encoded = token.encode().unwrap();
        println!("{}", encoded);
        let decoded = super::UserToken::decode(&encoded).unwrap();
        assert!(decoded.valid_signature(secret, NAMESPACE));
        assert_eq!(decoded, token);
    }
}
