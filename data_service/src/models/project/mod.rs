pub mod board;
pub mod pages;

// use juniper::GraphQLObject;
use crate::data_loader::Dataloader;
use async_graphql::{ComplexObject, Context, InputObject, Result as GQLResult, SimpleObject};
use diesel::{Associations, Identifiable, Queryable};
use serde::{Deserialize, Serialize};

use super::diesel_schema::*;
use super::Company;

#[derive(
    SimpleObject, Associations, Identifiable, Queryable, Serialize, Deserialize, Debug, Clone,
)]
#[belongs_to(Company)]
#[graphql(complex)]
pub struct Project {
    pub id: i32,
    pub company_id: i32,
    pub name: String,
}

#[ComplexObject]
impl Project {
    async fn boards<'ctx>(&self, ctx: &Context<'ctx>) -> GQLResult<Vec<Board>> {
        let loader = ctx.data_unchecked::<Dataloader>();
        let r = loader
            .project_boards_loader
            .load_one(self.id)
            .await?
            .unwrap_or_else(|| vec![]);
        Ok(r)
    }
    async fn board<'ctx>(&self, ctx: &Context<'ctx>, board_id: i32) -> GQLResult<Option<Board>> {
        let loader = ctx.data_unchecked::<Dataloader>();
        let r = loader.board_loader.load_one(board_id).await?;
        Ok(r)
    }
}
#[derive(Insertable, InputObject, Clone, Debug, Serialize, Deserialize)]
#[table_name = "projects"]
pub struct NewProject {
    pub name: String,
    pub company_id: i32,
}

pub use board::Board;
