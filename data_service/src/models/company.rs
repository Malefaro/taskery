use async_graphql::{ComplexObject, InputObject, Result as GQLResult, SimpleObject};
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

#[derive(Insertable, InputObject, Clone, Debug, Serialize, Deserialize)]
#[table_name = "companies"]
pub struct NewCompany {
    pub name: String,
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
#[table_name = "company_user_relations"]
pub struct CompanyUserRelation {
    pub id: i32,
    pub user_id: i32,
    pub company_id: i32,
}
