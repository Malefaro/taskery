pub mod mongo;
pub mod postgres;

use std::process::exit;

use super::models::{Company, User};
use async_trait::async_trait;

type DatabaseResult<T> = Result<T, Box<dyn std::error::Error>>;

#[async_trait]
pub trait Database {
    async fn get_user_by_id(&self, id: i32) -> DatabaseResult<Option<User>> {
        Ok(self.get_users_by_id_list(&[id]).await?.into_iter().next())
    }
    async fn get_users_by_id_list(&self, id_list: &[i32]) -> DatabaseResult<Vec<User>>;
    async fn get_company_by_id(&self, id: i32) -> DatabaseResult<Option<Company>> {
        Ok(self.get_companies_by_id_list(&[id]).await?.into_iter().next())
    }
    async fn get_companies_by_id_list(&self, id_list: &[i32]) -> DatabaseResult<Vec<Company>>;
}
