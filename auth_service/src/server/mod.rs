use crate::{
    proto::{auth::auth_service_server::AuthServiceServer, AuthServer},
    session_manager::SessionManager,
};

use tonic::transport;

pub struct Server<SM: SessionManager + Send + Sync + 'static> {
    pub port: i32,
    pub session_manager: SM,
}

impl<SM: SessionManager + Send + Sync + 'static> Server<SM> {
    pub fn new(port: i32, session_manager: SM) -> Self {
        Self {
            port,
            session_manager,
        }
    }
    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = format!("0.0.0.0:{}", self.port).parse().unwrap();
        let auth_service = AuthServer::new(self.session_manager);
        println!("Starting server at: {}", addr);
        transport::Server::builder()
            .add_service(AuthServiceServer::new(auth_service))
            .serve(addr)
            .await?;
        Ok(())
    }
}
