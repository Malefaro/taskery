use juniper::{EmptyMutation, EmptySubscription, GraphQLObject, RootNode};
use serde::{Deserialize, Serialize};

#[derive(GraphQLObject, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub(crate) password: String,
}

#[test]
fn tst() {
    let u = User{id: 1, email: "asd".to_string(), password: "qwe".to_string()};
    let schema = RootNode::new(u, EmptyMutation::<()>::new(), EmptySubscription::<()>::new());
    let r = schema.as_schema_language();
    println!("{}", r);
}


