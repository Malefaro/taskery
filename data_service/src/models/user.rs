use async_graphql::{
    dataloader::{DataLoader, Loader},
    ComplexObject, Context, Result as GQLResult, SimpleObject,
};
use diesel::{Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::{
    // data_loader::{Dataloader, UserID},
    database::postgres::PostgresDB,
};

use super::{diesel_schema::*, Company};
#[derive(SimpleObject, Identifiable, Queryable, Serialize, Deserialize, Debug, Clone)]
#[graphql(complex)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub is_admin: bool,
}
#[ComplexObject]
impl User {
    async fn companies<'ctx>(&self, ctx: &Context<'ctx>) -> GQLResult<Vec<Company>> {
        // let db = ctx.data_unchecked::<DataLoader<Dataloader<PostgresDB>>>();
        // // let db:&Dataloader<PostgresDB> = Dataloader::from_ctx(ctx);
        // let res = db.load_one(1 as UserID).await;
        // let db = ctx.data_unchecked::<ABC>();
        unimplemented!()
    }
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub password: String,
}
