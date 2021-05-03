pub mod column;
pub mod task;

use async_graphql::{SimpleObject, ComplexObject, Result as GQLResult};
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Identifiable, Associations};

use super::super::diesel_schema::*;
use super::Project;

#[derive(SimpleObject, Queryable, Associations, Identifiable, Serialize, Deserialize, Debug)]
#[graphql(complex)]
#[belongs_to(Project)]
pub struct Board {
    pub id: i32, 
    pub name: String, 
    pub project_id: i32,
}

#[ComplexObject]
impl Board {
    async fn columns(&self) -> GQLResult<BoardColumn> {
        unimplemented!()
    }
}

pub use column::BoardColumn;