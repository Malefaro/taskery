#[cfg(test)]
mod tests {
    use juniper::{EmptyMutation, EmptySubscription, RootNode};
    use models::User;
    use models::schema;


    #[test]
    fn tst() {
        // extern crate diesel;
        use schema::users::dsl::*;
        use diesel::prelude::*;
        let r = users.filter(email.eq("asd".to_string()));
        let u = User{id: 1, email: "asd".to_string(), password: "qwe".to_string()};
        let schema = RootNode::new(u, EmptyMutation::<()>::new(), EmptySubscription::<()>::new());
        let r = schema.as_schema_language();
        println!("{}", r);
    }

}