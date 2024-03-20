use proto::de::phyrone::kiwi::auth::auth_service_server::AuthService;
use proto::de::phyrone::kiwi::auth::{ValidateSessionRequest, ValidateSessionResponse};
use proto::tonic::{Request, Response, Status};

fn main() {
    
}


struct AuthServiceImpl;

impl AuthService for AuthServiceImpl{
    async fn validate_session(&self, request: Request<ValidateSessionRequest>) -> Result<Response<ValidateSessionResponse>, Status> {
        todo!()
    }
}