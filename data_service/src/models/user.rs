use serde::{Deserialize, Serialize};
use diesel::{Queryable, Identifiable, Insertable};
use async_graphql::{SimpleObject, ComplexObject, Result as GQLResult};

use super::{Company, diesel_schema::*};
#[derive(SimpleObject, Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[graphql(complex)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
}

#[ComplexObject]
impl User {
    async fn companies(&self) -> GQLResult<Company> {
        unimplemented!()
    }
}

#[derive(Insertable)]
#[table_name="users"]
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