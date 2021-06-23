// use juniper::GraphQLObject;
use async_graphql::{ComplexObject, Context, InputObject, Result as GQLResult, SimpleObject};
use diesel::{Associations, Identifiable, Queryable};
use serde::{Deserialize, Serialize};

use super::super::super::diesel_schema::*;
use super::Board;

use crate::data_loader::Dataloader;
use crate::models::board::task::Task;

#[derive(
    SimpleObject, Associations, Identifiable, Queryable, Serialize, Deserialize, Debug, Clone,
)]
#[graphql(complex)]
#[belongs_to(Board)]
pub struct BoardColumn {
    pub id: i32,
    pub name: String,
    pub board_id: i32,
}

#[ComplexObject]
impl BoardColumn {
    async fn tasks<'ctx>(&self, ctx: &Context<'ctx>) -> GQLResult<Vec<Task>> {
        let loader = ctx.data_unchecked::<Dataloader>();
        let r = loader
            .column_tasks_loader
            .load_one(self.id)
            .await?
            .unwrap_or_else(|| vec![]);
        Ok(r)
    }
    async fn task<'ctx>(&self, ctx: &Context<'ctx>, task_id: i32) -> GQLResult<Option<Task>> {
        let loader = ctx.data_unchecked::<Dataloader>();
        let r = loader.task_loader.load_one(task_id).await?;
        Ok(r)
    }
}

#[derive(Insertable, InputObject, Clone, Debug, Serialize, Deserialize)]
#[table_name = "board_columns"]
pub struct NewBoardColumn {
    pub name: String,
    pub board_id: i32,
}
