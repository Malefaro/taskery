use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Identifiable, Associations};

use crate::schema::*;
use crate::{Company, User};

#[derive(GraphQLObject, Queryable, Associations, Identifiable, Serialize, Deserialize, Debug)]
#[belongs_to(Company)]
#[belongs_to(User)]
pub struct Experience {
    pub id: i32, 
    pub total_exp: i32,
    pub user_id: i32,
    pub company_id: i32,
}
