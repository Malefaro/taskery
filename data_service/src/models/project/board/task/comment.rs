use chrono::NaiveDateTime;
// use juniper::GraphQLObject;
use async_graphql::{SimpleObject, ComplexObject, Result as GQLResult};
use serde::{Deserialize, Serialize};
use diesel::{Associations, Identifiable, Queryable};

use super::Task;
use super::super::super::super::User;
use super::super::super::super::diesel_schema::*;

#[derive(SimpleObject, Associations, Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[belongs_to(Task, foreign_key="task_id")]
#[belongs_to(User, foreign_key="author_id")]
pub struct TaskComment {
    pub id: i32, 
    pub text: String, 
    pub task_id: i32,
    pub author_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}


