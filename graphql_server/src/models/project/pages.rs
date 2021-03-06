use async_graphql::SimpleObject;
use diesel::{Associations, Identifiable, Queryable};
use serde::{Deserialize, Serialize};

use super::super::diesel_schema::*;
use super::Project;

#[derive(
    SimpleObject, Queryable, Associations, Identifiable, Serialize, Deserialize, Debug, Clone,
)]
#[belongs_to(Project)]
pub struct Page {
    pub id: i32,
    pub name: String,
    pub project_id: i32,
    pub text: String,
}
