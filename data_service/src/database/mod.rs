// pub mod mongo;
pub mod postgres;

use std::process::exit;

use crate::models::{NewCompany, NewUser};

use super::models::{Company, User};
use async_trait::async_trait;

type DatabaseResult<T> = Result<T, Box<dyn std::error::Error>>;

#[async_trait]
pub trait DatabaseRead {
    async fn get_user_by_id(&self, id: i32) -> DatabaseResult<Option<User>> {
        Ok(self.get_users_by_id_list(&[id]).await?.into_iter().next())
    }
    async fn get_users_by_id_list(&self, id_list: &[i32]) -> DatabaseResult<Vec<User>>;
    async fn get_company_by_id(&self, id: i32) -> DatabaseResult<Option<Company>> {
        Ok(self
            .get_companies_by_id_list(&[id])
            .await?
            .into_iter()
            .next())
    }
    async fn get_companies_by_id_list(&self, id_list: &[i32]) -> DatabaseResult<Vec<Company>>;
    async fn get_user_companies(&self, user_id: i32) -> DatabaseResult<Vec<Company>>;
    // async fn get_user_company(&self, user_id: i32, company_id: i32) -> DatabaseResult<Vec<Company>>;
}

#[async_trait]
pub trait DatabaseCreate {
    async fn create_user(&self, user: NewUser) -> DatabaseResult<User>;
    async fn create_company(&self, user_id: i32, company: NewCompany) -> DatabaseResult<Company>;
}

pub trait Database: DatabaseRead + DatabaseCreate {}

// impl<T> Database for T where T: DatabaseRead+DatabaseCreate {}
