use async_graphql::{ComplexObject, Result as GQLResult, SimpleObject};
use diesel::{Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

use super::{diesel_schema::*, Company};
#[derive(SimpleObject, Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[graphql(complex)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub is_admin: bool,
}

#[ComplexObject]
impl User {
    async fn companies(&self) -> GQLResult<Company> {
        unimplemented!()
    }
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub password: String,
}

// use crate::company::Company;
// #[graphql_object(context=Context)]
// impl User {
//     pub async fn id(&self) -> i32 {
//         self.id
//     }
//     pub async fn companies(&self, context: &Context) -> Vec<Company> {

//     }
// }
