
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