// use async_graphql::{ComplexObject, EmptyMutation, EmptySubscription, NewType, Object, Schema, SimpleObject};

// #[derive(NewType)]
// pub struct User(crate::models::User);
// struct QueryRoot;

// #[Object]
// impl QueryRoot {
//     async fn value(&self) -> User {
//         let u = crate::models::User{id:1, email:"asd".to_string(), password: "asd".to_string()};
//         User(u)
//     }
// }

// #[test] 
// fn test() {
//     let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).data("hello".to_string()).finish();
//     // let res = schema.execute("{ value }").await.into_result().unwrap().data;
//     println!("{}", schema.sdl());
//     // println!("{}", schema.federation_sdl());
// }
