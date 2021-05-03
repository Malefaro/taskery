pub mod comment;
pub mod history;

pub use comment::*;
pub use history::*;

use chrono::NaiveDateTime;
use async_graphql::{SimpleObject, ComplexObject, Result as GQLResult};
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Identifiable, Associations};

use super::BoardColumn;
use super::Board;
use super::super::super::User;
use super::super::super::diesel_schema::*;

// pub enum TaskStatus {
//     Done,
//     InProgress,
//     Pending,
// }
#[derive(SimpleObject, Associations, Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[graphql(complex)]
#[belongs_to(BoardColumn, foreign_key="column_id")]
#[belongs_to(User, foreign_key="author_id")]
pub struct Task {
    pub id: i32, 
    pub name: String, 
    pub text: String,
    // pub status: TaskStatus,
    pub resolved: bool,
    pub column_id: i32,
    pub author_id: i32,
    pub performer_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl Task {
    async fn comments(&self) -> GQLResult<self::comment::TaskComment> {
        unimplemented!()
    }
    async fn tags(&self) -> GQLResult<Tag> {
        unimplemented!()
    }
}

#[derive(SimpleObject, Associations, Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[belongs_to(User, foreign_key="author_id")]
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

#[derive(SimpleObject, Associations, Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[belongs_to(Tag)]
#[belongs_to(Task)]
pub struct TaskTagRelation {
    pub id: i32, 
    pub task_id: i32,
    pub tag_id: i32,
}