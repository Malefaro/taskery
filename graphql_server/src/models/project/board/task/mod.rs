pub mod comment;
pub mod history;

pub use comment::*;
pub use history::*;

use async_graphql::{
    dataloader::DataLoader, ComplexObject, Context, InputObject, Result as GQLResult, SimpleObject,
};
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Associations, Identifiable, Queryable};
use serde::{Deserialize, Serialize};

use super::super::super::diesel_schema::*;
use super::super::super::User;
use super::Board;
use super::BoardColumn;
use crate::data_loader::Dataloader;

#[derive(
    SimpleObject, Associations, Identifiable, Queryable, Serialize, Deserialize, Debug, Clone,
)]
#[graphql(complex)]
#[belongs_to(BoardColumn, foreign_key = "column_id")]
#[belongs_to(User, foreign_key = "author_id")]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub text: String,
    pub resolved: bool,
    pub column_id: i32,
    pub author_id: i32,
    pub performer_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl Task {
    async fn tags<'ctx>(&self, ctx: &Context<'ctx>) -> GQLResult<Vec<Tag>> {
        let loader = ctx.data_unchecked::<Dataloader>();
        let r = loader
            .task_tags_loader
            .load_one(self.id)
            .await?
            .unwrap_or_else(|| vec![]);
        Ok(r)
    }
    async fn comments<'ctx>(&self, ctx: &Context<'ctx>) -> GQLResult<Vec<TaskComment>> {
        let loader = ctx.data_unchecked::<Dataloader>();
        let r = loader
            .task_comments_loader
            .load_one(self.id)
            .await?
            .unwrap_or_else(|| vec![]);
        Ok(r)
    }
}

#[derive(
    SimpleObject, Associations, Identifiable, Queryable, Serialize, Deserialize, Debug, Clone,
)]
#[belongs_to(User, foreign_key = "author_id")]
#[belongs_to(Board)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub color: String,
    pub exp: i32,
    pub board_id: i32,
    pub author_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(
    SimpleObject, Associations, Identifiable, Queryable, Serialize, Deserialize, Debug, Clone,
)]
#[belongs_to(Tag)]
#[belongs_to(Task)]
pub struct TaskTagRelation {
    pub id: i32,
    pub task_id: i32,
    pub tag_id: i32,
}

#[derive(Insertable, InputObject, Clone, Debug, Serialize, Deserialize)]
#[table_name = "tasks"]
pub struct NewTask {
    pub name: String,
    pub column_id: i32,
    pub text: String,
    pub author_id: i32,
}

#[derive(Insertable, InputObject, Clone, Debug, Serialize, Deserialize)]
#[table_name = "tags"]
pub struct NewTag {
    pub name: String,
    pub color: String,
    pub exp: i32,
    pub board_id: i32,
    pub author_id: i32,
}

#[derive(AsChangeset, InputObject, Clone, Debug, Serialize, Deserialize)]
#[table_name = "tasks"]
pub struct TaskForm {
    pub name: Option<String>,
    pub text: Option<String>,
    pub resolved: Option<bool>,
    pub performer_id: Option<i32>,
}
