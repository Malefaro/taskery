use async_graphql::{
    dataloader::DataLoader, Context, EmptySubscription, Object, Result as GQLResult, Schema,
};
use serde::Serialize;

use crate::{
    data_loader::Dataloader,
    database::postgres::PostgresDB,
    models::{Company, NewCompany, NewUser, User},
};
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn user<'ctx>(&self, ctx: &Context<'ctx>, id: i32) -> GQLResult<Option<User>> {
        let db = ctx.data_unchecked::<Dataloader>();
        let r = db.0.get_user_by_id(id).await?;
        Ok(r)
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_user<'ctx>(&self, ctx: &Context<'ctx>, user: NewUser) -> GQLResult<User> {
        let db = ctx.data_unchecked::<Dataloader>();
        let r = db.0.create_user(user).await?;
        Ok(r)
    }
    async fn create_company<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        user_id: i32,
        company: NewCompany,
    ) -> GQLResult<Company> {
        let db = ctx.data_unchecked::<Dataloader>();
        let r = db.0.create_company(user_id, company).await?;
        Ok(r)
    }
}

#[actix_rt::test]
async fn test() {
    use async_graphql::{Request, Variables};
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(Dataloader(Box::pin(PostgresDB::new(
            "postgres://localhost/taskery",
        ))))
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
