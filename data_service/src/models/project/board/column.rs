// use juniper::GraphQLObject;
use async_graphql::{SimpleObject, ComplexObject, Result as GQLResult};
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Identifiable, Associations};

use super::Board;
use super::super::super::diesel_schema::*;

#[derive(SimpleObject, Associations, Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[graphql(complex)]
#[belongs_to(Board)]
pub struct BoardColumn {
    pub id: i32, 
    pub name: String, 
    pub board_id: i32,
}

#[ComplexObject]
impl BoardColumn {
    async fn tasks(&self) -> GQLResult<super::task::Task> {
        unimplemented!()
    }
}