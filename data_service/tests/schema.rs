// #[cfg(test)]
// mod tests {
//     use juniper::{EmptyMutation, EmptySubscription, RootNode};
//     use data_service::models::User;

//     #[test]
//     fn tst() {
//         // extern crate diesel;
//         use data_service::models::diesel_schema::users::dsl::*;
//         use diesel::prelude::*;
//         struct Query;
//         let r = users.filter(email.eq("asd".to_string()));
//         let u = User{id: 1, email: "asd".to_string(), password: "qwe".to_string()};
//         let schema = RootNode::new(u, EmptyMutation::<()>::new(), EmptySubscription::<()>::new());
//         let r = schema.as_schema_language();
//         println!("{}", r);
//     }

// }
#[actix_rt::test]
async fn test() {
    use async_graphql::{EmptySubscription, Schema};
    use async_graphql::{Request, Variables};
    use data_service::data_loader::Dataloader;
    use data_service::database::postgres::PostgresDB;
    use data_service::graphql::{mutation_root::MutationRoot, query_root::QueryRoot};
    let db = Box::pin(PostgresDB::new("postgres://localhost/taskery"));
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
