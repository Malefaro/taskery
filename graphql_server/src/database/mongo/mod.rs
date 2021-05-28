use super::Database;

pub struct MongoDB {}

#[async_trait::async_trait]
impl Database for MongoDB {
    async fn get_users_by_id_list(&self, id_list: &[i32]) -> super::DatabaseResult<Vec<crate::models::User>> {
        todo!()
    }

    async fn get_companies_by_id_list(&self, id_list: &[i32]) -> super::DatabaseResult<Vec<crate::models::Company>> {
        todo!()
    }
}
