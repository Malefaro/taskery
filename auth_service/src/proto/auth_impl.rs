pub mod auth {
    include!("auth.rs");
}

use auth::auth_service_server::AuthService;
use auth::{Nothing, Token, UserId};

#[derive(Default, Debug)]
pub struct AuthServer;

#[tonic::async_trait]
impl AuthService for AuthServer {
    async fn check(
        &self,
        request: tonic::Request<Token>,
    ) -> Result<tonic::Response<UserId>, tonic::Status> {
        unimplemented!()
    }
    async fn delete(
        &self,
        request: tonic::Request<Token>,
    ) -> Result<tonic::Response<Nothing>, tonic::Status> {
        unimplemented!()
    }
    async fn create(
        &self,
        request: tonic::Request<UserId>,
    ) -> Result<tonic::Response<Token>, tonic::Status> {
        println!("ASD");
        unimplemented!()
    }
}
