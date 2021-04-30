use std::marker::PhantomData;

use juniper::{FieldResult, graphql_interface, graphql_object};
use models::User;


pub struct QueryRoot;

// #[cfg(future="postgres")]
// pub struct Context{
//     pool: i32,
// }
// pub struct Context;
pub trait Database {
    fn get_obj(&self);
}
pub struct Context<DB: Database> {
    db: DB,

}
pub struct PSQL {
    pool: i32,

}
impl Database for PSQL {
    fn get_obj(&self) {
        
    }
}
pub struct Mongo {

}
impl Database for Mongo {
    fn get_obj(&self) {
        
    }
}


#[cfg(feature="postgres")]
type Ctx = Context<PSQL>;

#[cfg(feature="mongo")]
type Ctx = Context<Mongo>;

impl juniper::Context for Ctx{}

#[graphql_object(context=Ctx)]
impl QueryRoot {
    fn user(context: &Ctx, id: i32) -> FieldResult<User> {
        let c = context.db.get_obj();
        Ok(User{id:0, email: "asd".to_string(), password:"asd2".to_string()})
    }
}



struct MutationRoot;

#[test]
fn tst() {
    use juniper::{
        graphql_object, EmptyMutation, EmptySubscription, FieldResult, 
        GraphQLEnum, Variables, graphql_value, 
    };
    let ctx = Context{db:PSQL{pool:1}};
    type Schema = juniper::RootNode<'static, QueryRoot, EmptyMutation<Context<PSQL>>, EmptySubscription<Context<PSQL>>>;

    // Run the executor.
    let (res, _errors) = juniper::execute_sync(
        "query { user(id: 1){id, email, password} }",
        None,
        &Schema::new(QueryRoot, EmptyMutation::new(), EmptySubscription::new()),
        &Variables::new(),
        &ctx,
    ).unwrap();
    println!("{}", res);
    // Ensure the value matches.
    
}
 