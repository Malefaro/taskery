pub mod column;
pub mod task;

use async_graphql::{ComplexObject, Result as GQLResult, SimpleObject};
use diesel::{Associations, Identifiable, Queryable};
use serde::{Deserialize, Serialize};

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
