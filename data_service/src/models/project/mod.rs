pub mod board;
pub mod pages;

// use juniper::GraphQLObject;
use async_graphql::{SimpleObject, ComplexObject, Result as GQLResult};
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Identifiable, Associations};

use super::Company;
use super::diesel_schema::*;

#[derive(SimpleObject,Associations, Identifiable, Queryable, Serialize, Deserialize, Debug)]
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