use crate::{
    data_loader::Dataloader,
    models::{
        board::{
            column::NewBoardColumn,
            task::{NewTag, NewTask, Tag, Task, TaskForm},
            BoardColumn, NewBoard,
        },
        pages::Page,
        Auth, Board, Company, NewAuth, NewCompany, NewProject, NewUser, Project, User,
    },
};
use async_graphql::{Context, Error, Object, Result as GQLResult};
use auth_service::client::Client;
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn login<'ctx>(&self, ctx: &Context<'ctx>, auth: NewAuth) -> GQLResult<Auth> {
        let loader = ctx.data_unchecked::<Dataloader>();
        let r = loader
            .db
            .get_user_by_email_password(auth.email, auth.password)
            .await?;
        let u = r.ok_or(Error::new("Wrong email or password"))?;
        let mut auth_client = ctx.data_unchecked::<Client>().clone();
        let token = auth_client.sign_in(u.id).await?;
        Ok(Auth::new(u.id, token))
    }
    async fn logout<'ctx>(&self, ctx: &Context<'ctx>) -> GQLResult<String> {
        let auth = ctx.data_opt::<Auth>().ok_or(Error::new("Not Authorized"))?;
        let mut auth_client = ctx.data_unchecked::<Client>().clone();
        auth_client.logout(auth.token.clone()).await?;
        Ok("OK".to_string())
    }
    async fn create_user<'ctx>(&self, ctx: &Context<'ctx>, user: NewUser) -> GQLResult<User> {
        let loader = ctx.data_unchecked::<Dataloader>();
        let r = loader.db.create_user(user).await?;
        Ok(r)
    }
    async fn create_company<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        user_id: i32,
        company: NewCompany,
    ) -> GQLResult<Company> {
        let loader = ctx.data_unchecked::<Dataloader>();
        let r = loader.db.create_company(user_id, company).await?;
        Ok(r)
    }
    async fn create_project<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        project: NewProject,
    ) -> GQLResult<Project> {
        let loader = ctx.data_unchecked::<Dataloader>();
        let r = loader.db.create_project(project).await?;
        Ok(r)
    }
    async fn create_board<'ctx>(&self, ctx: &Context<'ctx>, board: NewBoard) -> GQLResult<Board> {
        let loader = ctx.data_unchecked::<Dataloader>();
        let r = loader.db.create_board(board).await?;
        Ok(r)
    }
    async fn create_board_column<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        column: NewBoardColumn,
    ) -> GQLResult<BoardColumn> {
        let loader = ctx.data_unchecked::<Dataloader>();
        let r = loader.db.create_column(column).await?;
        Ok(r)
    }
    async fn create_task<'ctx>(&self, ctx: &Context<'ctx>, task: NewTask) -> GQLResult<Task> {
        let loader = ctx.data_unchecked::<Dataloader>();
        let r = loader.db.create_task(task).await?;
        Ok(r)
    }
    async fn create_tag<'ctx>(&self, ctx: &Context<'ctx>, tag: NewTag) -> GQLResult<Tag> {
        let loader = ctx.data_unchecked::<Dataloader>();
        let r = loader.db.create_tag(tag).await?;
        Ok(r)
    }
    async fn modify_task<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        id: i32,
        task: TaskForm,
    ) -> GQLResult<Task> {
        let loader = ctx.data_unchecked::<Dataloader>();
        let r = loader.db.modify_task(id, task).await?;
        Ok(r)
    }
}
