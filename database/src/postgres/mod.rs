use actix_web::error::BlockingError;
use async_trait::async_trait;
use diesel::{QueryDsl, pg::PgConnection};
use diesel::r2d2::{ Pool, PooledConnection, ConnectionManager, PoolError };
use diesel::prelude::*;

use crate::Database;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;
// pub struct PostgresConnection(PgPooledConnection);
pub struct PostgresDB {
    pool: PgPool,
}

impl PostgresDB {
    fn new(url: &str) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(url);
        let pool = Pool::builder().build(manager).expect(&format!("Cannot connect with url: {}", url));
        Self {
            pool
        }
    }

    // fn con(&self) -> crate::DatabaseResult<PostgresConnection> {
    //     Ok(PostgresConnection(self.pool.get()?))
    // }
}
impl Clone for PostgresDB {
    fn clone(&self) -> Self {
        Self{pool: self.pool.clone()}
    }
}


// Execute blocking code in thread pool
async fn sync_to_async<F, I, E>(f: F) -> Result<I,BlockingError<E>>
where
    F: FnOnce() -> Result<I, E> + Send + 'static,
    I: Send + 'static,
    E: Send + std::fmt::Debug + 'static,
{
    use actix_web::web;
    web::block(f).await
}

#[async_trait]
impl Database for PostgresDB {
    async fn get_user_by_id(&self, id: i32) -> crate::DatabaseResult<models::User> {
        use models::schema::users::dsl::users;
        let c = self.pool.get()?;
        let res = sync_to_async(move || {
            users.find(id).first::<models::User>(&c)
        }).await?;
        Ok(res)
    }

    async fn get_users_by_id_list(&self, id_list: Vec<i32>) -> crate::DatabaseResult<Vec<models::User>> {
        let c = self.pool.get()?;
        use models::schema::users::dsl::*;
        let res = sync_to_async(move || {
            users
            .filter(id.eq_any(id_list))
            .load::<models::User>(&c)
        }).await?;
        return Ok(res);
    }

    async fn get_company_by_id(&self, id: i32) -> crate::DatabaseResult<models::Company> {
        todo!()
    }

    async fn get_companies_by_id_list(&self, id_list: Vec<i32>) -> crate::DatabaseResult<Vec<models::Company>> {
        todo!()
    }
}

#[test]
fn test() {
    use actix_web::rt::System;
    let mut s = System::new("test");
    let db = PostgresDB::new("postgres://:@localhost/taskery");
    let db_cloned = db.clone();
    let r = async move {db_cloned.get_user_by_id(1).await};
    let r2 = async move {db.get_user_by_id(2).await};
    // let r2 = db.get_user_by_id(1);
    let res = s.block_on(r);
    println!("{:?}", res);
    let res = s.block_on(r2);
    println!("{:?}", res);
}