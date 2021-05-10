pub mod auth {
    include!("auth.rs");
}

use crate::session_manager::SessionManager;
use auth::auth_service_server::AuthService;
use auth::{Nothing, Token, UserId};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
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
        let token = request.into_inner().id;
        self.session_manager
            .get_user_id_by_token(token.clone())
            .await
            .map_err(|_| Status::new(Code::Unauthenticated, format!("Token: {} not found", token)))
            .map(|user_id| Response::new(UserId { user_id }))
    }
    async fn delete(&self, request: Request<Token>) -> Result<Response<Nothing>, Status> {
        self.session_manager
            .delete_token(request.into_inner().id)
            .await
            .map_err(|err| Status::new(Code::NotFound, err.to_string()))
            .map(|_| Response::new(Nothing { dummy: true }))
    }
    async fn create(&self, request: Request<UserId>) -> Result<Response<Token>, Status> {
        let token: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();
        self.session_manager
            .set_token_for_user(token.clone(), request.into_inner().user_id)
            .await
            .map_err(|err| Status::new(Code::AlreadyExists, err.to_string()))
            .map(|_| Response::new(Token { id: token }))
    }
}
