use std::collections::HashMap;

use dataloader::BatchFn;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use webauthn_rs::prelude::RequestChallengeResponse;

#[derive(Debug, Serialize, Deserialize)]
pub enum WebAuthNChallengeData {
    LoginChallengeResponse(RequestChallengeResponse)
}

pub struct AuthManagerBatcher {
    database: DatabaseConnection,
    database_ro: DatabaseConnection,
}
impl BatchFn<u64, database::account::Model> for AuthManagerBatcher {
    async fn load(&mut self, keys: &[u64]) -> HashMap<u64, database::account::Model> {
        
        todo!()
    }
}

pub struct AuthManager {
    client: redis::Client,
}


impl AuthManager {
    fn a() {
        
    }
}

