use std::hash::Hash;

use base64::engine::general_purpose::URL_SAFE_NO_PAD as B64;
use base64::Engine;
use chrono::{DateTime, Utc};
use digest::Mac;
use jsonwebtoken::{Algorithm, TokenData};
use num::Zero;
use sea_orm::{ConnectionTrait, EntityTrait};
use serde::{Deserialize, Serialize};

use common::error_stack::ResultExt;
use common::{error_object, error_stack};

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

pub async fn check_user_token<C>(
    token: &str,
    database: &C,
    namespace: &str,
) -> Option<UserTokenPayload>
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

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Hash)]
pub struct SessionToken {
    #[serde(rename = "sub", with = "crate::utils::serde::base36")]
    user_id: u64,
    #[serde(rename = "exp", default, with = "chrono::serde::ts_seconds_option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    until: Option<DateTime<Utc>>,
    #[serde(rename = "nbf", default, with = "chrono::serde::ts_seconds_option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    since: Option<DateTime<Utc>>,
    #[serde(rename = "iat", with = "chrono::serde::ts_seconds")]
    issued_at: DateTime<Utc>,
    #[serde(rename = "jti", default, with = "crate::utils::serde::base36")]
    #[serde(skip_serializing_if = "Self::is_zero")]
    iteration: u64,
    #[serde(default, rename = "aud")]
    audiences: Vec<String>,
}

impl SessionToken {
    #[inline]
    fn is_zero(i: &u64) -> bool {
        i.is_zero()
    }

    const ALGORITHM: Algorithm = Algorithm::HS384;

    pub fn encode(&self, secret: &[u8]) -> error_stack::Result<String, TokenEncodeError> {
        let header = jsonwebtoken::Header::new(Self::ALGORITHM);
        let key = jsonwebtoken::EncodingKey::from_secret(secret);
        let token = jsonwebtoken::encode(&header, self, &key)
            .change_context(TokenEncodeError)
            .attach_printable("encode jwt token")?;
        Ok(token)
    }

    pub fn parse_no_validation(
        token: &str,
    ) -> error_stack::Result<TokenData<Self>, TokenDecodeError> {
        let key = jsonwebtoken::DecodingKey::from_secret(b"no secret");
        let mut validation = jsonwebtoken::Validation::new(Self::ALGORITHM);
        //skip signature validation
        validation.insecure_disable_signature_validation();
        validation.validate_aud = false;
        validation.validate_exp = false;
        validation.validate_nbf = false;
        let token = jsonwebtoken::decode::<Self>(token, &key, &validation)
            .change_context(TokenDecodeError)
            .attach_printable("decode jwt token")?;
        Ok(token)
    }

    pub fn parse(
        token: &str,
        secret: &[u8],
        audiences: &[impl ToString],
    ) -> error_stack::Result<TokenData<Self>, TokenDecodeError> {
        let key = jsonwebtoken::DecodingKey::from_secret(secret);
        let mut validation = jsonwebtoken::Validation::new(Self::ALGORITHM);
        validation.set_audience(audiences);
        validation.set_required_spec_claims(&["iat", "sub", "aud"]);
        validation.validate_exp = true;
        validation.validate_nbf = true;
        let token = jsonwebtoken::decode::<Self>(token, &key, &validation)
            .change_context(TokenDecodeError)
            .attach_printable("decode jwt token")?;
        Ok(token)
    }
}

impl Default for SessionToken {
    fn default() -> Self {
        Self {
            user_id: 0,
            until: None,
            since: None,
            issued_at: Utc::now(),
            iteration: 0,
            audiences: Vec::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use chrono::{Duration, Timelike, Utc};

    use crate::auth::SessionToken;

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

    #[test]
    pub fn test_jwt() {
        let token = SessionToken {
            user_id: 188287775496339456_u64,
            until: Some(Utc::now().with_nanosecond(0).unwrap() + Duration::hours(2)),
            issued_at: Utc::now().with_nanosecond(0).unwrap(),
            iteration: 125,
            ..Default::default()
        };

        let jwt = token.encode(b"secret").unwrap();
        println!("{}", jwt);
        let decoded = SessionToken::parse_no_validation(&jwt).unwrap();
        println!("{:?}", decoded.claims);
        assert_eq!(decoded.claims, token);
    }
}
