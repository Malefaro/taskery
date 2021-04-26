use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Identifiable};

use crate::schema::*;
#[derive(GraphQLObject, Identifiable, Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
}
