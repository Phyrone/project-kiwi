use std::hash::Hash;

use base64::Engine;
use chrono::{DateTime, Utc};
use digest::Mac;
use jsonwebtoken::{Algorithm, TokenData};
use num::Zero;
use sea_orm::{ConnectionTrait, EntityTrait};
use serde::{Deserialize, Serialize};

use common::{error_object, error_stack};
use common::error_stack::ResultExt;

type HmacSha256 = hmac::Hmac<sha3::Sha3_256>;

error_object!(TokenEncodeError, "error encoding session token");
error_object!(TokenDecodeError, "error reading session token");


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
    #[serde(default, rename = "aud")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    audiences: Vec<String>,
}

impl SessionToken {
    const ALGORITHM: Algorithm = Algorithm::HS384;

    pub fn encode(&self, secret: &[u8]) -> error_stack::Result<String, TokenEncodeError> {
        let header = jsonwebtoken::Header::new(Self::ALGORITHM);
        let key = jsonwebtoken::EncodingKey::from_secret(secret);
        let token = jsonwebtoken::encode(&header, self, &key)
            .change_context(TokenEncodeError)?;
        Ok(token)
    }

    pub fn parse(
        token: &str,
        secret: Option<&[u8]>,
        audiences: &[impl ToString],
        validate_time: bool,
    ) -> error_stack::Result<TokenData<Self>, TokenDecodeError> {
        let key = jsonwebtoken::DecodingKey::from_secret(secret.unwrap_or(b"no secret"));

        let mut validation = jsonwebtoken::Validation::new(Self::ALGORITHM);

        let check_audicences = !audiences.is_empty();
        if check_audicences {
            validation.set_audience(audiences);
            validation.set_required_spec_claims(&["iat", "sub", "aud"]);
        } else {
            validation.set_required_spec_claims(&["iat", "sub"]);
        }
        validation.validate_aud = check_audicences;


        validation.validate_exp = validate_time;
        validation.validate_nbf = validate_time;
        if secret.is_none() {
            validation.insecure_disable_signature_validation();
        }
        let token = jsonwebtoken::decode::<Self>(token, &key, &validation)
            .change_context(TokenDecodeError)?;
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
            audiences: Vec::new(),
        }
    }
}


#[cfg(test)]
mod test {
    use chrono::{Duration, Timelike, Utc};

    use crate::auth::SessionToken;

    #[test]
    pub fn test_jwt() {
        let token = SessionToken {
            user_id: 188287775496339456_u64,
            since: Some(Utc::now().with_nanosecond(0).unwrap()),
            until: Some(Utc::now().with_nanosecond(0).unwrap() + Duration::hours(2)),
            issued_at: Utc::now().with_nanosecond(0).unwrap(),
            ..Default::default()
        };

        let jwt = token.encode(b"secret").unwrap();
        println!("{}", jwt);
        let decoded = SessionToken::parse(&jwt, Some(b"secret"), &Vec::<String>::new(), true).unwrap();
        println!("{:?}", decoded.claims);
        assert_eq!(decoded.claims, token);
    }
}
