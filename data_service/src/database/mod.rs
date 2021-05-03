pub mod postgres;
pub mod mongo;

use async_trait::async_trait;
use super::models::{Company, User};

type DatabaseResult<T> = Result<T, Box<dyn std::error::Error>>;

#[async_trait]
pub trait Database {
    async fn get_user_by_id(&self, id: i32) -> DatabaseResult<User>;
    async fn get_users_by_id_list(&self, id_list: Vec<i32>) -> DatabaseResult<Vec<User>>;
    async fn get_company_by_id(&self, id: i32) -> DatabaseResult<Company>;
    async fn get_companies_by_id_list(&self, id_list: Vec<i32>) -> DatabaseResult<Vec<Company>>;
}
