// use juniper::GraphQLObject;
use async_graphql::{ComplexObject, Result as GQLResult, SimpleObject};
use diesel::{Associations, Identifiable, Queryable};
use serde::{Deserialize, Serialize};

use super::super::super::diesel_schema::*;
use super::Board;

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
