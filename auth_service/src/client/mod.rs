use std::error::Error;

use tonic::{transport::Channel, Request};

use crate::proto::auth::auth_service_client::AuthServiceClient;
use crate::proto::auth::{Token, UserId};
pub struct Client {
    client: AuthServiceClient<Channel>,
}
impl Clone for Client {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
        }
    }
}

impl Client {
    pub async fn connect(url: String) -> Result<Self, Box<dyn Error>> {
        let client = AuthServiceClient::connect(url).await?;
        Ok(Self { client })
    }

    pub async fn sign_in(&mut self, user_id: i32) -> Result<String, Box<dyn Error>> {
        let request = Request::new(UserId { user_id });
        Ok(self
            .client
            .create(request)
            .await?
            .map(|x| x.id)
            .into_inner())
    }
    pub async fn logout(&mut self, token: String) -> Result<(), Box<dyn Error>> {
        let request = Request::new(Token { id: token });
        Ok(self.client.delete(request).await?.map(|_| ()).into_inner())
    }
    pub async fn check_auth(&mut self, token: String) -> Result<i32, Box<dyn Error>> {
        let request = Request::new(Token { id: token });
        Ok(self
            .client
            .check(request)
            .await?
            .map(|u| u.user_id)
            .into_inner())
    }
}
