use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Associations};

use crate::User;
use crate::schema::*;

#[derive(GraphQLObject, Queryable, Identifiable, Serialize, Deserialize, Debug)]
#[table_name="companies"]
pub struct Company {
    pub id: i32,
    pub name: String,
    // users: Vec<User>,   
}

#[derive(GraphQLObject, Queryable, Identifiable, Associations,Serialize, Deserialize, Debug)]
#[belongs_to(User)]
#[belongs_to(Company)]
pub struct CompanyUserRelation {
    pub id: i32, 
    pub user_id: i32,
    pub company_id: i32,
}