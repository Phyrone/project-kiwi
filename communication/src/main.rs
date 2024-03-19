use proto::de::phyrone::kiwi::auth::{ValidateSessionRequest, ValidateSessionResponse};
use proto::tonic;
use proto::tonic::{Request, Response, Status};

fn main() {
    println!("Hello, world!");
}


#[derive(Debug, Default, Clone)]
pub struct AuthService {}

#[tonic::async_trait]
impl proto::de::phyrone::kiwi::auth::auth_service_server::AuthService for AuthService {
    async fn validate_session(&self, request: Request<ValidateSessionRequest>) -> Result<Response<ValidateSessionResponse>, Status> {

        todo!()
    }
}