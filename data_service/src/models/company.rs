use async_graphql::{SimpleObject, ComplexObject, Result as GQLResult};
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Associations};

use crate::models::User;
use crate::models::diesel_schema::*;

use super::Project;

#[derive(SimpleObject, Queryable, Identifiable, Serialize, Deserialize, Debug)]
#[table_name="companies"]
#[graphql(complex)]
pub struct Company {
    pub id: i32,
    pub name: String,
    // users: Vec<User>,   
}

#[ComplexObject]
impl Company {
    async fn projects(&self) -> GQLResult<Project> {
        unimplemented!()
    }
}

#[derive(SimpleObject, Queryable, Identifiable, Associations,Serialize, Deserialize, Debug)]
#[belongs_to(User)]
#[belongs_to(Company)]
pub struct CompanyUserRelation {
    pub id: i32, 
    pub user_id: i32,
    pub company_id: i32,
}