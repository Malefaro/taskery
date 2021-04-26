use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Identifiable, Associations};

use crate::schema::*;
use crate::Project;

#[derive(GraphQLObject, Queryable, Associations, Identifiable, Serialize, Deserialize, Debug)]
#[belongs_to(Project)]
pub struct Page {
    pub id: i32, 
    pub name: String, 
    pub project_id: i32,
}