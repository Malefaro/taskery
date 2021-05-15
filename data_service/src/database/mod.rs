// pub mod mongo;
pub mod postgres;

use std::{collections::HashMap, pin::Pin};

use crate::models::{Board, NewCompany, NewProject, NewUser, Project, board::{BoardColumn, NewBoard, column::NewBoardColumn, task::{NewTag, NewTask, Tag, Task, TaskComment, TaskForm}}, pages::Page};

use super::models::{Company, User};
use async_trait::async_trait;

type DatabaseResult<T> = Result<T, Box<dyn std::error::Error>>;

#[async_trait]
pub trait DatabaseRead {
    async fn get_users_by_id_list(&self, id_list: &[i32]) -> DatabaseResult<Vec<User>>;
    async fn get_companies_by_id_list(&self, id_list: &[i32]) -> DatabaseResult<Vec<Company>>;
    async fn get_projects_by_id_list(&self, id_list: &[i32]) -> DatabaseResult<Vec<Project>>;
    async fn get_boards_by_id_list(&self, id_list: &[i32]) -> DatabaseResult<Vec<Board>>;
    async fn get_pages_by_id_list(&self, id_list: &[i32]) -> DatabaseResult<Vec<Page>>;
    async fn get_tasks_by_id_list(&self, id_list: &[i32]) -> DatabaseResult<Vec<Task>>;

    // related
    async fn get_users_companies(&self, users_ids: &[i32]) -> DatabaseResult<HashMap<i32, Vec<Company>>>;
    async fn get_companies_projects(&self, companies_ids: &[i32]) -> DatabaseResult<HashMap<i32, Vec<Project>>>;
    async fn get_projects_boards(&self, projects_ids: &[i32]) -> DatabaseResult<HashMap<i32, Vec<Board>>>;
    async fn get_projects_pages(&self, projects_ids: &[i32]) -> DatabaseResult<HashMap<i32, Vec<Page>>>;
    async fn get_boards_columns(&self, boards_ids: &[i32]) -> DatabaseResult<HashMap<i32, Vec<BoardColumn>>>;
    async fn get_columns_tasks(&self, columns_ids: &[i32]) -> DatabaseResult<HashMap<i32, Vec<Task>>>;
    async fn get_tasks_comments(&self, tasks_ids: &[i32]) -> DatabaseResult<HashMap<i32, Vec<TaskComment>>>;
    async fn get_tasks_tags(&self, tasks_ids: &[i32]) -> DatabaseResult<HashMap<i32, Vec<Tag>>>;
}

#[async_trait]
pub trait DatabaseCreate {
    async fn create_user(&self, user: NewUser) -> DatabaseResult<User>;
    async fn create_company(&self, user_id: i32, company: NewCompany) -> DatabaseResult<Company>;
    async fn create_project(&self, project: NewProject) -> DatabaseResult<Project>;
    async fn create_board(&self, board: NewBoard) -> DatabaseResult<Board>;
    async fn create_column(&self, column: NewBoardColumn) -> DatabaseResult<BoardColumn>;
    async fn create_task(&self, task: NewTask) -> DatabaseResult<Task>;
    async fn create_tag(&self, tag: NewTag) -> DatabaseResult<Tag>;
}

#[async_trait]
pub trait DatabaseModify {
    async fn modify_task(&self, task: TaskForm) -> DatabaseResult<User>;
    async fn set_tags_for_task(&self, task_id: i32, tags_ids: Vec<i32>) -> DatabaseResult<User>;
}

pub trait Database: DatabaseRead + DatabaseCreate + DatabaseModify + DatabaseClone {}

// Magic trick to make trait object clonable in our case (Pin<Box<...>>)
// I have no idea why its works ( I mean how compiler resolve impl DatabaseClone for Database + Clone where Database require DatabaseClone)
// https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-boxed-trait-object
pub trait DatabaseClone {
    fn clone_pin_box(&self) -> Pin<Box<dyn Database + Send + Sync>>;
}

impl <T: Send+Sync> DatabaseClone for T where T: 'static + Database + Clone {
    fn clone_pin_box(&self) -> Pin<Box<dyn Database + Send + Sync>> {
        Box::pin(self.clone())
    }
}

impl Clone for Pin<Box<dyn Database + Send + Sync>> {
    fn clone(&self) -> Pin<Box<dyn Database+Send+Sync>> {
        self.clone_pin_box()
    }
}

