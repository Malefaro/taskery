pub mod board;
pub mod pages;

// use juniper::GraphQLObject;
use async_graphql::{ComplexObject, Result as GQLResult, SimpleObject};
use diesel::{Associations, Identifiable, Queryable};
use serde::{Deserialize, Serialize};

use super::diesel_schema::*;
use super::Company;

#[derive(SimpleObject, Associations, Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[belongs_to(Company)]
#[graphql(complex)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub company_id: i32,
}

#[ComplexObject]
impl Project {
    async fn boards(&self) -> GQLResult<Board> {
        unimplemented!()
    }
}

pub use board::Board;
