use async_graphql::{FieldError, dataloader::Loader};
use std::{collections::HashMap, pin::Pin};

use crate::{database::Database, models::{Company, User}};

pub struct UserDataloader(pub Pin<Box<dyn Database+Send+Sync>>);

#[async_trait::async_trait]
impl Loader<i32> for UserDataloader {
    type Value = User;

    type Error = FieldError;

    async fn load(&self, keys: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        let res = self
            .0
            .get_users_by_id_list(keys)
            .await?;
        let m: HashMap<i32, Self::Value> = res
            .into_iter()
            .map(|model| (model.id, model))
            .collect();
        Ok(m)
    }
}

pub struct UserCompaniesLoader(pub Pin<Box<dyn Database+Send+Sync>>);

#[async_trait::async_trait]
impl Loader<i32> for UserCompaniesLoader {
    type Value = Vec<Company>;

    type Error = FieldError;

    async fn load(&self, keys: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        let res = self
            .0
            .get_users_companies(keys)
            .await?;
        Ok(res)
    }
}



