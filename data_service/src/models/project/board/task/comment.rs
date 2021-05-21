use chrono::NaiveDateTime;
// use juniper::GraphQLObject;
use async_graphql::{ComplexObject, Result as GQLResult, SimpleObject};
use diesel::{Associations, Identifiable, Queryable};
use serde::{Deserialize, Serialize};

use super::super::super::super::diesel_schema::*;
use super::super::super::super::User;
use super::Task;

#[derive(
    SimpleObject, Associations, Identifiable, Queryable, Serialize, Deserialize, Debug, Clone,
)]
#[belongs_to(Task, foreign_key = "task_id")]
#[belongs_to(User, foreign_key = "author_id")]
pub struct TaskComment {
    pub id: i32,
    pub text: String,
    pub task_id: i32,
    pub author_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
