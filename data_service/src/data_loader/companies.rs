use std::{collections::HashMap, pin::Pin};

use async_graphql::{FieldError, dataloader::Loader};

use crate::{database::Database, models::Company};

pub struct CompaniesLoader(pub Pin<Box<dyn Database+Send+Sync>>);

#[async_trait::async_trait]
impl Loader<i32> for CompaniesLoader {
    type Value = Company;

    type Error = FieldError;

    async fn load(&self, keys: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        let res = self
            .0
            .get_companies_by_id_list(keys)
            .await?;
        let m: HashMap<i32, Self::Value> = res
            .into_iter()
            .map(|model| (model.id, model))
            .collect();
        Ok(m)
    }
}