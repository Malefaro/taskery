use async_graphql::{
    dataloader::DataLoader, Context, EmptySubscription, Object, Result as GQLResult, Schema,
};
use serde::Serialize;

use crate::{
    data_loader::Dataloader,
    database::postgres::PostgresDB,
    models::{Company, NewCompany, NewUser, User, Project, Board, pages::Page},
};
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn user<'ctx>(&self, ctx: &Context<'ctx>, id: i32) -> GQLResult<Option<User>> {
        let loader = ctx.data_unchecked::<Dataloader>();
        let r = loader.user_loader.load_one(id).await?;
        Ok(r)
    }
    async fn companies<'ctx>(&self, ctx: &Context<'ctx>, user_id: Option<i32> /* this field can be accessed though request(from ctx.data)*/) -> GQLResult<Vec<Company>> {
        // check user_id. if None -> check ctx.data::<Auth>() -> if no error
        todo!()
    }
    async fn projects<'ctx>(&self, ctx: &Context<'ctx>, company_id: i32) -> GQLResult<Vec<Project>> {
        todo!()
    }
    async fn boards<'ctx>(&self, ctx: &Context<'ctx>, project_id: i32) -> GQLResult<Vec<Board>> {
        todo!()
    }
    async fn board<'ctx>(&self, ctx: &Context<'ctx>, board_id: i32) -> GQLResult<Option<Board>> {
        let loader = ctx.data_unchecked::<Dataloader>();
        let r = loader.board_loader.load_one(board_id).await?;
        Ok(r)
    }
    async fn pages<'ctx>(&self, ctx: &Context<'ctx>, project_id: i32) -> GQLResult<Vec<Page>> {
        todo!()
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
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
}

#[actix_rt::test]
async fn test() {
    use async_graphql::{Request, Variables};
    let db = Box::pin(PostgresDB::new(
            "postgres://localhost/taskery",
        ));
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(Dataloader::new(db))
        .finish();
    // println!("{}", schema.sdl());
    let r = Request::new(r#"query {user(id: 1) {email, companies{id,name}}}"#);

    // let r = Request::new(
    //     r#"mutation CreateUser($newUser: NewUser!) {
    //         createUser(user: $newUser){id, email, password, isAdmin}
    //     }"#,
    // )
    // .variables(Variables::from_json(
    //     serde_json::from_str(&r#"{"newUser":{"email": "asd@m.r", "password": "asd"}}"#.to_string()).unwrap()
    // ));

    // let r = Request::new(
    //     r#"mutation CreateCompany($user_id: Int!, $newCompany: NewCompany!) {
    //         createCompany(userId: $user_id, company: $newCompany){id, name}
    //     }"#,
    // )
    // .variables(Variables::from_json(
    //     serde_json::from_str(&r#"{"newCompany":{"name": "failed company"}, "user_id": 2}"#.to_string()).unwrap()
    // ));
    println!("{:?}", r);
    let res = schema.execute(r).await.into_result();
    // let tp = res.unwrap().data;
    match res {
        Ok(r) => println!("{}", r.data),
        Err(err) => err.into_iter().for_each(|err| println!("{}", err)),
    };
    // println!("{:?}", res);
    // println!("{}", schema.federation_sdl());
}
