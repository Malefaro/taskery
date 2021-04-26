use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Identifiable, Associations};

use crate::project::Board;
use crate::schema::*;

#[derive(GraphQLObject, Associations, Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[belongs_to(Board)]
pub struct BoardColumn {
    pub id: i32, 
    pub name: String, 
    pub board_id: i32,
}
