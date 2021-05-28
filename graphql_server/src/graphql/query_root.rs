use crate::{
    data_loader::Dataloader,
    models::{pages::Page, Board, Company, Project, User},
};
use async_graphql::{Context, Error, Object, Result as GQLResult};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn user<'ctx>(&self, ctx: &Context<'ctx>, id: i32) -> GQLResult<Option<User>> {
        let loader = ctx.data_unchecked::<Dataloader>();
        let r = loader.user_loader.load_one(id).await?;
        Ok(r)
    }
    async fn companies<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        user_id: Option<i32>, /* this field can be accessed though request(from ctx.data)*/
    ) -> GQLResult<Vec<Company>> {
        let loader = ctx.data_unchecked::<Dataloader>();
        let user_id = user_id.ok_or(Error::new("user_id not supplied"))?;
        let r = loader
            .user_companies_loader
            .load_one(user_id)
            .await?
            .unwrap_or_else(|| vec![]);
        Ok(r)
    }
    async fn projects<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        company_id: i32,
    ) -> GQLResult<Vec<Project>> {
        let loader = ctx.data_unchecked::<Dataloader>();
        let r = loader
            .company_projects_loader
            .load_one(company_id)
            .await?
            .unwrap_or_else(|| vec![]);
        Ok(r)
    }
    async fn boards<'ctx>(&self, ctx: &Context<'ctx>, project_id: i32) -> GQLResult<Vec<Board>> {
        let loader = ctx.data_unchecked::<Dataloader>();
        let r = loader
            .project_boards_loader
            .load_one(project_id)
            .await?
            .unwrap_or_else(|| vec![]);
        Ok(r)
    }
    async fn board<'ctx>(&self, ctx: &Context<'ctx>, board_id: i32) -> GQLResult<Option<Board>> {
        let loader = ctx.data_unchecked::<Dataloader>();
        let r = loader.board_loader.load_one(board_id).await?;
        Ok(r)
    }
    async fn pages<'ctx>(&self, ctx: &Context<'ctx>, project_id: i32) -> GQLResult<Vec<Page>> {
        let loader = ctx.data_unchecked::<Dataloader>();
        let r = loader
            .project_pages_loader
            .load_one(project_id)
            .await?
            .unwrap_or_else(|| vec![]);
        Ok(r)
    }
}
