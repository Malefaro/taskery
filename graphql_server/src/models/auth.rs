use async_graphql::{
    guard::Guard, ComplexObject, Context, InputObject, Result as GQLResult, SimpleObject,
};
use serde::{Deserialize, Serialize};

use super::User;
use crate::data_loader::Dataloader;

// pub struct Token {
//     pub user_id: i32,
//     pub key: String,
// }

#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct NewAuth {
    pub email: String,
    pub password: String,
}

#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
#[graphql(complex)]
pub struct Auth {
    pub user_id: i32,
    pub token: String,
    #[graphql(skip)]
    user: Option<User>,
}
impl Auth {
    pub fn new(user_id: i32, token: String) -> Self {
        Self {
            user_id,
            token,
            user: None,
        }
    }
}
#[ComplexObject]
impl Auth {
    async fn user<'ctx>(&self, ctx: &Context<'ctx>) -> GQLResult<Option<User>> {
        let loader = ctx.data_unchecked::<Dataloader>();
        let r = loader.user_loader.load_one(self.user_id).await?;
        Ok(r)
    }
}

pub struct AuthGuard {
    pub is_auth: bool,
}

#[async_trait::async_trait]
impl Guard for AuthGuard {
    async fn check(&self, ctx: &Context<'_>) -> GQLResult<()> {
        // if ctx.data_opt::<Auth>().is_some() {
        //     Ok(())
        // } else {
        //     Err("Forbidden".into())
        // }
        ctx.data_opt::<Auth>().map(|_| ()).ok_or("Forbidden".into())
    }
}
