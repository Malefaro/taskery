pub mod auth {
    include!("auth.rs");
}

use crate::session_manager::SessionManager;
use auth::auth_service_server::AuthService;
use auth::{Nothing, Token, UserId};
use tonic::{Code, Request, Response, Status};

#[derive(Default, Debug)]
pub struct AuthServer<SM> {
    session_manager: SM,
}

impl<SM> AuthServer<SM>
where
    SM: SessionManager + Send + Sync + 'static,
{
    pub fn new(session_manager: SM) -> Self {
        Self { session_manager }
    }
}

#[tonic::async_trait]
impl<SM> AuthService for AuthServer<SM>
where
    SM: SessionManager + Send + Sync + 'static,
{
    async fn check(&self, request: Request<Token>) -> Result<Response<UserId>, Status> {
        self.session_manager
            .get_user_id_by_token(request.into_inner().id)
            .await
            .map_err(|err| Status::new(Code::Unauthenticated, err.to_string()))
            .map(|user_id| Response::new(UserId { user_id }))
    }
    async fn delete(&self, request: Request<Token>) -> Result<Response<Nothing>, Status> {
        unimplemented!()
    }
    async fn create(&self, request: Request<UserId>) -> Result<Response<Token>, Status> {
        println!("ASD");
        unimplemented!()
    }
}
