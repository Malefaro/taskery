use juniper::{GraphQLObject};
use serde::{Deserialize, Serialize};
use diesel::Queryable;

#[derive(GraphQLObject, Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
}
