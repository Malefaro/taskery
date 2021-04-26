use chrono::NaiveDateTime;
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};
use diesel::{Associations, Identifiable, Queryable};

use crate::project::board::task::Task;
use crate::User;
use crate::schema::*;

#[derive(GraphQLObject, Associations, Identifiable, Queryable, Serialize, Deserialize, Debug)]
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


