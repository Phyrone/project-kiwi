use serde::{Deserialize, Serialize};
use webauthn_rs::prelude::RequestChallengeResponse;

#[derive(Debug, Serialize, Deserialize)]
pub enum  WebAuthNChallengeData{
    LoginChallengeResponse(RequestChallengeResponse)
    
    
}

pub struct AuthManager{
    
}
