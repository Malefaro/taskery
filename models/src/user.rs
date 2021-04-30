use juniper::{GraphQLObject, graphql_object};
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Identifiable, Insertable};

use crate::schema::*;
#[derive(GraphQLObject, Identifiable, Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
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