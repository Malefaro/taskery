use std::collections::HashMap;

use actix_web::error::BlockingError;
use async_trait::async_trait;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use diesel::{pg::PgConnection, QueryDsl};
use diesel::{prelude::*, query_dsl::InternalJoinDsl};

use crate::{database::Database, models::{Board, Company, CompanyUserRelation, NewCompany, NewUser, Project, User, board::{BoardColumn, task::{Tag, Task, TaskComment}}, pages::Page}};

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

macro_rules! get_models_many {
    ($self:ident, $id_list:ident, $dsl_name:ident, $model:ident) => {
        let c = $self.pool.get()?;
        use crate::models::diesel_schema::$dsl_name::dsl::*;
        // need copy it due to sync_to_async (need static lifetime for closure)
        let id_list: Vec<i32> = $id_list.iter().copied().collect();
        let res = sync_to_async(move || $dsl_name.filter(id.eq_any(id_list)).load::<$model>(&c)).await?;
        return Ok(res)
    };
}
macro_rules! get_related_models {
    ($self:ident, $id_list:ident, $table:ident, $through_table:ident, $field:ident, $model:ident) => {
        let c = $self.pool.get()?;
        use crate::models::diesel_schema::$table as c;
        use crate::models::diesel_schema::$through_table as through;
        let id_list: Vec<i32> = $id_list.iter().copied().collect();
        let res = sync_to_async(move || {
            through::table
                .inner_join(c::table)
                .select((through::$field, c::all_columns))
                .filter(through::$field.eq_any(id_list))
                .load::<(i32, $model)>(&c)
        })
        .await?;
        let mut m: HashMap<i32, Vec<$model>> = HashMap::with_capacity(res.len());
        res.into_iter().for_each(|(id, c)| {
            m.entry(id).and_modify(|e| e.push(c.clone())).or_insert(vec![c]);
        });
        return Ok(m)
    };
}
#[async_trait]
impl DatabaseRead for PostgresDB {
    async fn get_users_by_id_list(&self, id_list: &[i32]) -> DatabaseResult<Vec<User>> {
        use crate::models::diesel_schema::users::*;
        get_models_many!(self, id_list, users, User);
    }

    async fn get_companies_by_id_list(&self, id_list: &[i32]) -> DatabaseResult<Vec<Company>> {
        get_models_many!(self, id_list, companies, Company);
    }

    async fn get_projects_by_id_list(&self, id_list: &[i32]) -> DatabaseResult<Vec<Project>> {
        get_models_many!(self, id_list, projects, Project);
    }

    async fn get_boards_by_id_list(&self, id_list: &[i32]) -> DatabaseResult<Vec<Board>> {
        get_models_many!(self, id_list, boards, Board);
    }

    async fn get_pages_by_id_list(&self, id_list: &[i32]) -> DatabaseResult<Vec<Page>> {
        get_models_many!(self, id_list, pages, Page);
    }

    async fn get_users_companies(&self, id_list: &[i32]) -> DatabaseResult<HashMap<i32, Vec<Company>>> {
        get_related_models!(self, id_list, companies, company_user_relations, company_id, Company);
    }

    async fn get_companies_projects(&self, companies_ids: &[i32]) -> DatabaseResult<HashMap<i32, Vec<Project>>> {
        get_related_models!(self, companies_ids, projects, company_project_relations, company_id, Project);
        // todo!()
    }

    async fn get_projects_boards(&self, projects_ids: &[i32]) -> DatabaseResult<HashMap<i32, Vec<Board>>> {
        get_related_models!(self, projects_ids, boards, project_board_relations, project_id, Board);
    }

    async fn get_projects_pages(&self, projects_ids: &[i32]) -> DatabaseResult<HashMap<i32, Vec<Page>>> {
        get_related_models!(self, projects_ids, pages, project_pages_relations, project_id, Page);
    }

    async fn get_boards_columns(&self, boards_ids: &[i32]) -> DatabaseResult<HashMap<i32, Vec<BoardColumn>>> {
        get_related_models!(self, boards_ids, boards, board_column_relations, board_id, BoardColumn);
    }

    async fn get_columns_tasks(&self, columns_ids: &[i32]) -> DatabaseResult<HashMap<i32, Vec<Task>>> {
        get_related_models!(self, columns_ids, tasks, column_task_relations, column_id, Task);
    }

    async fn get_tasks_comments(&self, tasks_ids: &[i32]) -> DatabaseResult<HashMap<i32, Vec<TaskComment>>> {
        get_related_models!(self, tasks_ids, task_comments, task_comment_relations, task_id, TaskComment);
    }

    async fn get_tasks_tags(&self, tasks_ids: &[i32]) -> DatabaseResult<HashMap<i32, Vec<Tag>>> {
        get_related_models!(self, tasks_ids, tags, task_tag_relations, task_id, Tag);
    }

    async fn get_tasks_by_id_list(&self, id_list: &[i32]) -> DatabaseResult<Vec<Task>> {
        get_models_many!(self, id_list, tasks, Task);
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

    async fn create_project(&self, project: crate::models::NewProject) -> DatabaseResult<Project> {
        todo!()
    }

    async fn create_board(&self, board: crate::models::board::NewBoard) -> DatabaseResult<Board> {
        todo!()
    }

    async fn create_column(&self, column: crate::models::board::column::NewBoardColumn) -> DatabaseResult<BoardColumn> {
        todo!()
    }

    async fn create_task(&self, task: crate::models::board::task::NewTask) -> DatabaseResult<Task> {
        todo!()
    }

    async fn create_tag(&self, tag: crate::models::board::task::NewTag) -> DatabaseResult<Tag> {
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
