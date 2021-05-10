use std::error::Error;

#[async_trait::async_trait]
pub trait SessionManager {
    async fn get_user_id_by_token(&self, token: &str) -> Result<i32, Box<dyn Error>>;
    async fn set_token_for_user(
        &self,
        token: &str,
        user_id: i32,
    ) -> Result<(), Box<dyn Error>>;
    async fn delete_token(&self, token: &str) -> Result<(), Box<dyn Error>>;
}

pub mod redis;
