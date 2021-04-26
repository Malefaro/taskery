pub mod board;
pub mod pages;

use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Identifiable, Associations};

use crate::Company;
use crate::schema::*;

#[derive(GraphQLObject,Associations, Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[belongs_to(Company)]
pub struct Project {
    pub id: i32, 
    pub name: String, 
    pub company_id: i32,
}

pub use board::Board;