use async_graphql::SimpleObject;
use diesel::{Associations, Identifiable, Queryable};
use serde::{Deserialize, Serialize};

use super::diesel_schema::*;
use super::{Company, User};

#[derive(
    SimpleObject, Queryable, Associations, Identifiable, Serialize, Deserialize, Debug, Clone,
)]
#[belongs_to(Company)]
#[belongs_to(User)]
pub struct Experience {
    pub id: i32,
    pub total_exp: i32,
    pub user_id: i32,
    pub company_id: i32,
}
