pub mod column;
pub mod task;

use crate::data_loader::Dataloader;
use async_graphql::{ComplexObject, Context, InputObject, Result as GQLResult, SimpleObject};
use diesel::{Associations, Identifiable, Queryable};
use serde::{Deserialize, Serialize};

use super::super::diesel_schema::*;
use super::Project;

#[derive(
    SimpleObject, Queryable, Associations, Identifiable, Serialize, Deserialize, Debug, Clone,
)]
#[graphql(complex)]
#[belongs_to(Project)]
pub struct Board {
    pub id: i32,
    pub name: String,
    pub project_id: i32,
}

#[ComplexObject]
impl Board {
    async fn columns<'ctx>(&self, ctx: &Context<'ctx>) -> GQLResult<Vec<BoardColumn>> {
        let loader = ctx.data_unchecked::<Dataloader>();
        let r = loader
            .board_columns_loader
            .load_one(self.id)
            .await?
            .unwrap_or_else(|| vec![]);
        Ok(r)
    }
}

#[derive(Insertable, InputObject, Clone, Debug, Serialize, Deserialize)]
#[table_name = "boards"]
pub struct NewBoard {
    pub name: String,
    pub project_id: i32,
}

pub use column::BoardColumn;
