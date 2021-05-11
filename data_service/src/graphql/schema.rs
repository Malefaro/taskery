use async_graphql::{Context, Object, Result as GQLResult, dataloader::DataLoader};

use crate::{database::postgres::PostgresDB, models::{Company, User}};
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn users<'ctx>(&self, ctx: &Context<'ctx>) -> GQLResult<Vec<User>> {
        unimplemented!()
    }
    async fn user<'ctx>(&self, ctx: &Context<'ctx>, id: i32) -> GQLResult<Option<User>> {
        // let db = ctx.data_unchecked::<DataLoader<Dataloader<PostgresDB>>>();
        // let res = db.load_one(id).await?;
        // Ok(res)
        unimplemented!()
    }
    async fn companies<'ctx>(&self, ctx: &Context<'ctx>, user_id: i32) -> GQLResult<Vec<Company>> {
        unimplemented!()
    }
    async fn company<'ctx>(&self, ctx: &Context<'ctx>, company_id: i32) -> GQLResult<User> {
        unimplemented!()
    }
}
// use std::marker::PhantomData;

// use juniper::{FieldResult, graphql_interface, graphql_object};
// use crate::models::User;

// pub struct QueryRoot;

// // #[cfg(future="postgres")]
// // pub struct Context{
// //     pool: i32,
// // }
// // pub struct Context;
// pub trait Database {
//     fn get_obj(&self);
// }
// pub struct Context<DB: Database> {
//     db: DB,

// }
// pub struct PSQL {
//     pool: i32,

// }
// impl Database for PSQL {
//     fn get_obj(&self) {

//     }
// }
// pub struct Mongo {

// }
// impl Database for Mongo {
//     fn get_obj(&self) {

//     }
// }

// #[cfg(feature="postgres")]
// type Ctx = Context<PSQL>;

// #[cfg(feature="mongo")]
// type Ctx = Context<Mongo>;

// impl juniper::Context for Ctx{}

// #[graphql_object(context=Ctx)]
// impl QueryRoot {
//     fn user(context: &Ctx, id: i32) -> FieldResult<User> {
//         let c = context.db.get_obj();
//         Ok(User{id:0, email: "asd".to_string(), password:"asd2".to_string()})
//     }
// }

// struct MutationRoot;

// #[test]
// fn tst() {
//     use juniper::{
//         graphql_object, EmptyMutation, EmptySubscription, FieldResult,
//         GraphQLEnum, Variables, graphql_value,
//     };
//     let ctx = Context{db:PSQL{pool:1}};
//     type Schema = juniper::RootNode<'static, QueryRoot, EmptyMutation<Context<PSQL>>, EmptySubscription<Context<PSQL>>>;
//     let schema = Schema::new(QueryRoot, EmptyMutation::new(), EmptySubscription::new());
//     println!("{}", schema.as_schema_language());
//     // Run the executor.
//     let (res, _errors) = juniper::execute_sync(
//         "query { user(id: 1){id, email, password} }",
//         None,
//         &Schema::new(QueryRoot, EmptyMutation::new(), EmptySubscription::new()),
//         &Variables::new(),
//         &ctx,
//     ).unwrap();
//     println!("{}", res);
//     // Ensure the value matches.

// }
