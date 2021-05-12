// use std::collections::HashMap;

// use actix_web::web::Data;
// use dataloader::{non_cached::Loader, BatchFn};

// use crate::{
//     database::Database,
//     models::{Company, User},
// };

// pub struct Batcher<DB> {
//     db: DB,
// }
// #[async_trait::async_trait]
// impl<DB> BatchFn<i32, User> for Batcher<DB>
// where
//     DB: Database + Sync + Send + 'static,
// {
//     async fn load(&mut self, keys: &[i32]) -> HashMap<i32, User> {
//         unimplemented!()
//     }
// }
// #[async_trait::async_trait]
// impl<DB> BatchFn<i32, Company> for Batcher<DB>
// where
//     DB: Database + Sync + Send + 'static,
// {
//     async fn load(&mut self, keys: &[i32]) -> HashMap<i32, Company> {
//         unimplemented!()
//     }
// }
// pub type ModelLoader<T, DB> = Loader<i32, T, Batcher<DB>>;

// pub struct Storage<DB> {
//     db: DB,
// }
// impl<DB> Storage<DB>
// where
//     DB: Database + Clone + Sync + Send + 'static,
// {
//     // создавать Loader внутри resolver'a бесполезно =/
//     pub fn get_user_loader(&self) -> ModelLoader<User, DB> {
//         Loader::new(Batcher {
//             db: self.db.clone(),
//         })
//     }
//     pub async fn tst(&self) {
//         let l = self.get_user_loader();
//         let u = l.load(1).await;
//     }
// }

/* -----------------------------------------------------------------------------*/
/* -----------------------------------------------------------------------------*/
/* -----------------------------------------------------------------------------*/

use actix_web::web::Data;
use async_graphql::{dataloader::Loader, Context, FieldError};
use std::{any::Any, collections::HashMap, error::Error, pin::Pin};

use crate::{database::Database, models::User};
use crate::{
    database::{postgres::PostgresDB, DatabaseRead},
    models::Company,
};

// pub struct Dataloader<DB: Database + Send + Sync + 'static> {
//     // #[cfg(feature = "postgres")]
//     // inner: PostgresDB,

//     // #[cfg(feature = "mongo")]
//     // inner: MongoDB,
//     inner: DB,
// }

pub struct Dataloader(pub Pin<Box<dyn Database + Send + Sync>>);

// impl<DB: Database + Send + Sync + 'static> Dataloader<DB> {
//     pub fn new(inner: DB) -> Self {
//         Self { inner }
//     }
//     pub fn from_ctx<'a, 'ctx>(ctx: &'a Context<'ctx>) -> &'a Self {
//         ctx.data_unchecked()
//     }
// }

// pub struct Dataloader(Pin<Box<dyn Database>>);

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct UserID(i32);

#[async_trait::async_trait]
// impl<DB: Database + Send + Sync + 'static> Loader<UserID> for Dataloader<DB> {
impl Loader<UserID> for Dataloader {
    type Value = User;

    type Error = FieldError;

    async fn load(&self, keys: &[UserID]) -> Result<HashMap<UserID, Self::Value>, Self::Error> {
        let res = self
            .0
            // .inner
            .get_users_by_id_list(&keys.iter().cloned().map(|i| i.0).collect::<Vec<i32>>())
            .await?;
        let m: HashMap<UserID, Self::Value> = res
            .into_iter()
            .map(|model| (UserID(model.id), model))
            .collect();
        Ok(m)
        // todo!()
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct CompanyID(i32);

#[async_trait::async_trait]
// impl<DB: Database + Send + Sync + 'static> Loader<CompanyID> for Dataloader<DB> {
impl Loader<CompanyID> for Dataloader {
    type Value = Company;

    type Error = FieldError;

    async fn load(
        &self,
        keys: &[CompanyID],
    ) -> Result<HashMap<CompanyID, Self::Value>, Self::Error> {
        let res = self
            .0
            .get_companies_by_id_list(&keys.iter().cloned().map(|i| i.0).collect::<Vec<i32>>())
            .await?;
        let m: HashMap<CompanyID, Self::Value> = res
            .into_iter()
            .map(|model| (CompanyID(model.id), model))
            .collect();
        Ok(m)
        // todo!()
    }
}
// #[actix_web::rt::test]
// async fn test() {
//     let l = Dataloader {
//         inner: PostgresDB::new(""),
//     };
//     let r = l.load(&[1 as UserID]);
// }

#[test]
fn test() {
    // pub struct Dataloader(Pin<Box<dyn Database + Send + Sync>>);
    let db1 = Dataloader(Box::pin(PostgresDB::new("")));
    // let db2=Dataloader(Box::pin(MongoDB{}));
    use std::any::TypeId;
    fn get_id<D: Any + Send + Sync>(v: D) -> TypeId {
        TypeId::of::<D>()
    }
    // assert_eq!(get_id(db1), get_id(db2));
}
