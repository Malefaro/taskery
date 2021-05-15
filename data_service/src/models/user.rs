use async_graphql::{
    dataloader::{DataLoader, Loader},
    ComplexObject, Context, InputObject, Result as GQLResult, SimpleObject,
};
use diesel::{Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::{data_loader::Dataloader, database::postgres::PostgresDB};

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
        let loader = ctx.data_unchecked::<Dataloader>();
        let r = loader.user_companies_loader.load_one(self.id).await?.unwrap_or_else(|| vec![]);
        Ok(r)
    }
    async fn company<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        company_id: i32,
    ) -> GQLResult<Option<Company>> {
        // TODO: maybe it should be private. So need check for access(user exists in this company)
        let loader = ctx.data_unchecked::<Dataloader>();
        let r = loader.company_loader.load_one(company_id).await?;
        Ok(r)
    }
}

#[derive(Insertable, InputObject, Clone, Debug, Serialize, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub password: String,
}
