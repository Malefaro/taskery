use async_graphql::{ComplexObject, Result as GQLResult, SimpleObject};
use diesel::{Associations, Queryable};
use serde::{Deserialize, Serialize};

use crate::models::diesel_schema::*;
use crate::models::User;

use super::Project;

#[derive(SimpleObject, Queryable, Identifiable, Serialize, Deserialize, Debug, Clone)]
#[table_name = "companies"]
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

#[derive(SimpleObject, Queryable, Identifiable, Associations, Serialize, Deserialize, Debug)]
#[belongs_to(User)]
#[belongs_to(Company)]
pub struct CompanyUserRelation {
    pub id: i32,
    pub user_id: i32,
    pub company_id: i32,
}
