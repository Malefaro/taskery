use std::collections::HashMap;

use actix_web::error::BlockingError;
use async_trait::async_trait;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use diesel::{pg::PgConnection, QueryDsl};
use diesel::{prelude::*, query_dsl::InternalJoinDsl};

use crate::{
    database::Database,
    models::{Company, CompanyUserRelation, NewCompany, NewUser, User},
};

use super::{DatabaseCreate, DatabaseModify, DatabaseRead, DatabaseResult};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;
// pub struct PostgresConnection(PgPooledConnection);
pub struct PostgresDB {
    pool: PgPool,
}

impl PostgresDB {
    pub fn new(url: &str) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(url);
        let pool = Pool::builder()
            .build(manager)
            .expect(&format!("Cannot connect with url: {}", url));
        Self { pool }
    }

    // fn con(&self) -> crate::DatabaseResult<PostgresConnection> {
    //     Ok(PostgresConnection(self.pool.get()?))
    // }
}
impl Clone for PostgresDB {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
        }
    }
}

// Execute blocking code in thread pool
pub async fn sync_to_async<F, I, E>(f: F) -> Result<I, BlockingError<E>>
where
    F: FnOnce() -> Result<I, E> + Send + 'static,
    I: Send + 'static,
    E: Send + std::fmt::Debug + 'static,
{
    use actix_web::web;
    web::block(f).await
}

#[async_trait]
impl DatabaseRead for PostgresDB {
    async fn get_users_by_id_list(&self, id_list: &[i32]) -> DatabaseResult<Vec<User>> {
        let c = self.pool.get()?;
        use crate::models::diesel_schema::users::dsl::*;
        // need copy it due to sync_to_async (need static lifetime for closure)
        let id_list: Vec<i32> = id_list.iter().copied().collect();
        let res = sync_to_async(move || users.filter(id.eq_any(id_list)).load::<User>(&c)).await?;
        return Ok(res);
    }

    // TODO: create macros for this.
    async fn get_companies_by_id_list(&self, id_list: &[i32]) -> DatabaseResult<Vec<Company>> {
        let c = self.pool.get()?;
        use crate::models::diesel_schema::companies::dsl::*;
        // need copy it due to sync_to_async (need static lifetime for closure)
        let id_list: Vec<i32> = id_list.iter().copied().collect();
        let res =
            sync_to_async(move || companies.filter(id.eq_any(id_list)).load::<Company>(&c)).await?;
        return Ok(res);
    }

    async fn get_projects_by_id_list(&self, id_list: &[i32]) -> DatabaseResult<Vec<crate::models::Project>> {
        todo!()
    }

    async fn get_boards_by_id_list(&self, id_list: &[i32]) -> DatabaseResult<Vec<crate::models::Board>> {
        todo!()
    }

    async fn get_pages_by_id_list(&self, id_list: &[i32]) -> DatabaseResult<Vec<crate::models::pages::Page>> {
        todo!()
    }

    async fn get_users_companies(&self, id_list: &[i32]) -> DatabaseResult<HashMap<i32, Vec<Company>>> {
        let c = self.pool.get()?;
        use crate::models::diesel_schema::companies as c;
        use crate::models::diesel_schema::company_user_relations as cur;
        let id_list: Vec<i32> = id_list.iter().copied().collect();
        let res = sync_to_async(move || {
            cur::table
                .inner_join(c::table.on(cur::company_id.eq(c::id)))
                .select((cur::user_id, c::all_columns))
                .filter(cur::user_id.eq_any(id_list))
                .load::<(i32, Company)>(&c)
        })
        .await?;
        let mut m: HashMap<i32, Vec<Company>> = HashMap::with_capacity(res.len());
        res.into_iter().for_each(|(id, c)| {
            m.entry(id).and_modify(|e| e.push(c.clone())).or_insert(vec![c]);
        });
        Ok(m)
    }

    async fn get_companies_projects(&self, companies_ids: &[i32]) -> DatabaseResult<HashMap<i32, Vec<crate::models::Project>>> {
        todo!()
    }

    async fn get_projects_boards(&self, projects_ids: &[i32]) -> DatabaseResult<HashMap<i32, Vec<crate::models::Board>>> {
        todo!()
    }

    async fn get_projects_pages(&self, projects_ids: &[i32]) -> DatabaseResult<HashMap<i32, Vec<crate::models::pages::Page>>> {
        todo!()
    }

    async fn get_boards_columns(&self, boards_ids: &[i32]) -> DatabaseResult<HashMap<i32, Vec<crate::models::board::BoardColumn>>> {
        todo!()
    }

    async fn get_columns_tasks(&self, columns_ids: &[i32]) -> DatabaseResult<HashMap<i32, Vec<crate::models::board::task::Task>>> {
        todo!()
    }

    async fn get_tasks_comments(&self, tasks_ids: &[i32]) -> DatabaseResult<HashMap<i32, Vec<crate::models::board::task::TaskComment>>> {
        todo!()
    }

    async fn get_tasks_tags(&self, tasks_ids: &[i32]) -> DatabaseResult<HashMap<i32, Vec<crate::models::board::task::Tag>>> {
        todo!()
    }

    async fn get_tasks_by_id_list(&self, id_list: &[i32]) -> DatabaseResult<Vec<crate::models::board::task::Task>> {
        todo!()
    }
}
#[async_trait]
impl DatabaseCreate for PostgresDB {
    async fn create_user(&self, user: NewUser) -> DatabaseResult<User> {
        let c = self.pool.get()?;
        use crate::models::diesel_schema::users;
        let res = sync_to_async(move || {
            diesel::insert_into(users::table)
                .values(&user)
                .get_result::<User>(&c)
        })
        .await?;
        Ok(res)
    }

    async fn create_company(&self, user_id: i32, company: NewCompany) -> DatabaseResult<Company> {
        let con = self.pool.get()?;
        use crate::models::diesel_schema::companies;
        use crate::models::diesel_schema::company_user_relations as cur;
        let res = sync_to_async(move || {
            con.transaction(|| {
                let res = diesel::insert_into(companies::table)
                    .values(&company)
                    .get_result::<Company>(&con)
                    .and_then(|c| {
                        diesel::insert_into(cur::table)
                            .values((cur::user_id.eq(user_id), cur::company_id.eq(c.id)))
                            .get_result::<CompanyUserRelation>(&con)?;
                        Ok(c)
                    });
                res
            })
        })
        .await?;
        Ok(res)
    }

    async fn create_project(&self, project: crate::models::NewProject) -> DatabaseResult<crate::models::Project> {
        todo!()
    }

    async fn create_board(&self, board: crate::models::board::NewBoard) -> DatabaseResult<crate::models::Board> {
        todo!()
    }

    async fn create_column(&self, column: crate::models::board::column::NewBoardColumn) -> DatabaseResult<crate::models::board::BoardColumn> {
        todo!()
    }

    async fn create_task(&self, task: crate::models::board::task::NewTask) -> DatabaseResult<crate::models::board::task::Task> {
        todo!()
    }

    async fn create_tag(&self, tag: crate::models::board::task::NewTag) -> DatabaseResult<crate::models::board::task::Tag> {
        todo!()
    }
}

#[async_trait]
impl DatabaseModify for PostgresDB {
    async fn modify_task(&self, task: crate::models::board::task::TaskForm) -> DatabaseResult<User> {
        todo!()
    }

    async fn set_tags_for_task(&self, task_id: i32, tags_ids: Vec<i32>) -> DatabaseResult<User> {
        todo!()
    }
}
impl Database for PostgresDB {}

// #[test]
// fn test() {
//     use actix_web::rt::System;
//     let mut s = System::new("test");
//     let db = PostgresDB::new("postgres://:@localhost/taskery");
//     fn db_check(db: impl Database) {}
//     db_check(db.clone());
//     let db_cloned = db.clone();
//     let r = async move { db_cloned.get_user_by_id(1).await };
//     let r2 = async move { db.get_user_by_id(2).await };
//     // let r2 = db.get_user_by_id(1);
//     let res = s.block_on(r);
//     println!("{:?}", res);
//     let res = s.block_on(r2);
//     println!("{:?}", res);
// }
