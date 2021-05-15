#[macro_export]
macro_rules! loader {
    ($name:ident, $model: ident, $method: ident) => {
        // use async_graphql::{dataloader::Loader, FieldError};
        // use std::collections::HashMap;
        // use std::pin::Pin;
        // use crate::models::*;
        // use crate::database::Database;

        pub struct $name(pub Pin<Box<dyn Database+Send+Sync>>);

        #[async_trait::async_trait]
        impl Loader<i32> for $name {
            type Value = $model;

            type Error = FieldError;

            async fn load(&self, keys: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
                let res = self
                    .0
                    .$method(keys)
                    .await?;
                let m: HashMap<i32, Self::Value> = res
                    .into_iter()
                    .map(|model| (model.id, model))
                    .collect();
                Ok(m)
            }
        }
    };
}

#[macro_export]
macro_rules! loader_related {
    ($name:ident, $model: ident, $method: ident) => {
        pub struct $name(pub Pin<Box<dyn Database+Send+Sync>>);

        #[async_trait::async_trait]
        impl Loader<i32> for $name {
            type Value = Vec<$model>;

            type Error = FieldError;

            async fn load(&self, keys: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
                let res = self
                    .0
                    .$method(keys)
                    .await?;
                Ok(res)
            }
        }
    };
}


#[macro_export]
macro_rules! create_dataloader {
    ( $dataloader_name:ident, $(($field_name: ident, $loader: ident)),+ ) => {
        pub struct $dataloader_name {
            pub db: Pin<Box<dyn Database + Send + Sync>>,
            $(
                pub $field_name: DataLoader<$loader>,
            )+
        }
        impl $dataloader_name {
            pub fn new(db: Pin<Box<dyn Database+Send+Sync>>) -> Self {
                Self{
                    $(
                        $field_name: DataLoader::new($loader(db.clone())),
                    )+
                    db, 
                }
            }
        }
    }
}