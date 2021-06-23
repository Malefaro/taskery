use std::collections::HashMap;

use actix_web::error::BlockingError;
use async_trait::async_trait;
use diesel::prelude::*;
use diesel::{pg::PgConnection, QueryDsl};
use diesel::{
    query_builder::InsertStatement,
    query_dsl::LoadQuery,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};

use crate::{
    database::Database,
    models::{
        board::{
            column::NewBoardColumn,
            task::{NewTag, NewTask, Tag, Task, TaskComment, TaskForm, TaskTagRelation},
            BoardColumn, NewBoard,
        },
        pages::Page,
        Board, Company, CompanyUserRelation, NewCompany, NewProject, NewUser, Project, User,
    },
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

macro_rules! get_models_many {
    ($self:ident, $id_list:ident, $dsl_name:ident, $model:ident) => {
        let c = $self.pool.get()?;
        use crate::models::diesel_schema::$dsl_name::dsl::*;
        // need copy it due to sync_to_async (need static lifetime for closure)
        let id_list: Vec<i32> = $id_list.iter().copied().collect();
        let res =
            sync_to_async(move || $dsl_name.filter(id.eq_any(id_list)).load::<$model>(&c)).await?;
        return Ok(res)
    };
}
macro_rules! get_related_models {
    ($self:ident, $id_list:ident, $table:ident, $field:ident, $model:ident) => {
        let c = $self.pool.get()?;
        use crate::models::diesel_schema::$table as t;
        // need copy it due to sync_to_async (need static lifetime for closure)
        let id_list: Vec<i32> = $id_list.iter().copied().collect();
        let res = sync_to_async(move || {
            t::table
                .filter(t::$field.eq_any(id_list))
                .load::<$model>(&c)
        })
        .await?;
        let mut m: HashMap<i32, Vec<$model>> = HashMap::with_capacity(res.len());
        res.into_iter().for_each(|obj| {
            m.entry(obj.$field)
                .and_modify(|e| e.push(obj.clone()))
                .or_insert(vec![obj]);
        });
        return Ok(m)
    };
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
            m.entry(id)
                .and_modify(|e| e.push(c.clone()))
                .or_insert(vec![c]);
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

    async fn get_users_companies(
        &self,
        id_list: &[i32],
    ) -> DatabaseResult<HashMap<i32, Vec<Company>>> {
        get_related_models!(
            self,
            id_list,
            companies,
            company_user_relations,
            company_id,
            Company
        );
    }

    async fn get_companies_projects(
        &self,
        companies_ids: &[i32],
    ) -> DatabaseResult<HashMap<i32, Vec<Project>>> {
        get_related_models!(self, companies_ids, projects, company_id, Project);
        // todo!()
    }

    async fn get_projects_boards(
        &self,
        projects_ids: &[i32],
    ) -> DatabaseResult<HashMap<i32, Vec<Board>>> {
        get_related_models!(self, projects_ids, boards, project_id, Board);
    }

    async fn get_projects_pages(
        &self,
        projects_ids: &[i32],
    ) -> DatabaseResult<HashMap<i32, Vec<Page>>> {
        get_related_models!(self, projects_ids, pages, project_id, Page);
    }

    async fn get_boards_columns(
        &self,
        boards_ids: &[i32],
    ) -> DatabaseResult<HashMap<i32, Vec<BoardColumn>>> {
        get_related_models!(self, boards_ids, board_columns, board_id, BoardColumn);
    }

    async fn get_columns_tasks(
        &self,
        columns_ids: &[i32],
    ) -> DatabaseResult<HashMap<i32, Vec<Task>>> {
        get_related_models!(self, columns_ids, tasks, column_id, Task);
    }

    async fn get_tasks_comments(
        &self,
        tasks_ids: &[i32],
    ) -> DatabaseResult<HashMap<i32, Vec<TaskComment>>> {
        get_related_models!(self, tasks_ids, task_comments, task_id, TaskComment);
    }

    async fn get_tasks_tags(&self, tasks_ids: &[i32]) -> DatabaseResult<HashMap<i32, Vec<Tag>>> {
        get_related_models!(self, tasks_ids, tags, task_tag_relations, task_id, Tag);
    }

    async fn get_tasks_by_id_list(&self, id_list: &[i32]) -> DatabaseResult<Vec<Task>> {
        get_models_many!(self, id_list, tasks, Task);
    }

    async fn get_user_by_email_password(
        &self,
        email: String,
        password: String,
    ) -> DatabaseResult<Option<User>> {
        let c = self.pool.get()?;
        use crate::models::diesel_schema::users;
        let r = sync_to_async(move || {
            users::table
                .filter(users::email.eq(email).and(users::password.eq(password)))
                .first::<User>(&c)
                .optional()
        })
        .await?;
        Ok(r)
    }
}

async fn create_model<T, M, R>(table: T, model: M, c: PgPooledConnection) -> DatabaseResult<R>
where
    T: Table + Send + Sync + 'static,
    M: diesel::Insertable<T> + Send + Sync + 'static,
    // InsertStatement<T, M::Values>: ExecuteDsl<PgConnection>,
    R: Send + Sync + 'static,
    InsertStatement<T, M::Values>: LoadQuery<PgPooledConnection, R>,
    // R: Queryable<T, PgConnection>
{
    let r =
        sync_to_async(move || diesel::insert_into(table).values(model).get_result::<R>(&c)).await?;
    Ok(r)
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

    async fn create_project(&self, project: NewProject) -> DatabaseResult<Project> {
        Ok(create_model(
            crate::models::diesel_schema::projects::table,
            project,
            self.pool.get()?,
        )
        .await?)
    }

    async fn create_board(&self, board: NewBoard) -> DatabaseResult<Board> {
        Ok(create_model(
            crate::models::diesel_schema::boards::table,
            board,
            self.pool.get()?,
        )
        .await?)
    }

    async fn create_column(&self, column: NewBoardColumn) -> DatabaseResult<BoardColumn> {
        Ok(create_model(
            crate::models::diesel_schema::board_columns::table,
            column,
            self.pool.get()?,
        )
        .await?)
    }

    async fn create_task(&self, task: NewTask) -> DatabaseResult<Task> {
        Ok(create_model(
            crate::models::diesel_schema::tasks::table,
            task,
            self.pool.get()?,
        )
        .await?)
    }

    async fn create_tag(&self, tag: NewTag) -> DatabaseResult<Tag> {
        Ok(create_model(
            crate::models::diesel_schema::tags::table,
            tag,
            self.pool.get()?,
        )
        .await?)
    }
}

#[async_trait]
impl DatabaseModify for PostgresDB {
    async fn modify_task(&self, id: i32, task: TaskForm) -> DatabaseResult<Task> {
        let c = self.pool.get()?;
        use crate::models::diesel_schema::tasks;
        let r = sync_to_async(move || {
            diesel::update(tasks::table)
                .filter(tasks::id.eq(id))
                .set(task)
                .get_result::<Task>(&c)
        })
        .await?;
        Ok(r)
    }

    async fn set_tags_for_task(
        &self,
        task_id: i32,
        tags_ids: Vec<i32>,
    ) -> DatabaseResult<Vec<TaskTagRelation>> {
        let c = self.pool.get()?;
        use crate::models::diesel_schema::task_tag_relations as ttr;
        let rels = tags_ids
            .into_iter()
            .map(|id| (ttr::tag_id.eq(id), ttr::task_id.eq(task_id)))
            .collect::<Vec<_>>();
        let r = sync_to_async(move || {
            diesel::insert_into(ttr::table)
                .values(&rels)
                .get_results::<TaskTagRelation>(&c)
        })
        .await?;
        Ok(r)
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
