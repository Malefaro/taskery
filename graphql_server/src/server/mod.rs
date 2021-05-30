use actix_web::{guard, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_actix_web::{Request, Response};
use auth_service::client::Client;
use std::pin::Pin;
use std::sync::Mutex;

use crate::{
    data_loader::Dataloader,
    database::Database,
    graphql::{mutation_root::MutationRoot, query_root::QueryRoot},
    models::Auth,
};
pub struct Server {
    db: Box<dyn Database + Send + Sync + 'static>,
    auth_client: Client,
    port: i32,
}
pub type MySchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

async fn index(
    schema: web::Data<MySchema>,
    auth_client: web::Data<Mutex<Client>>,
    req: Request,
    http_request: HttpRequest,
) -> Response {
    let token = http_request
        .headers()
        .get("Authorization")
        .and_then(|header_value| {
            header_value.to_str().ok().map(|s| {
                let start_index = "Bearer ".len();
                let token = s[start_index..s.len()].to_string();
                token
            })
        });
    let auth = match token {
        Some(token) => {
            let mut client = auth_client.lock().unwrap();
            let user_id = client.check_auth(token.clone()).await.ok();
            match user_id {
                Some(user_id) => Some(Auth::new(user_id, token)),
                None => None,
            }
        }
        None => None,
    };
    let mut req = req.into_inner();
    if let Some(auth) = auth {
        req = req.data(auth);
    }
    schema.execute(req).await.into()
}

async fn index_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        )))
}

impl Server {
    pub fn new(db: impl Database + 'static + Send + Sync, auth_client: Client, port: i32) -> Self {
        Self {
            db: Box::new(db),
            auth_client,
            port,
        }
    }

    pub async fn run(&self) -> std::io::Result<()> {
        println!("Starting server at port {}", self.port);
        let client = self.auth_client.clone();
        let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
            .data(Dataloader::new(self.db.clone()))
            .data(client.clone())
            .finish();
        HttpServer::new(move || {
            App::new()
                .data(schema.clone())
                .data(Mutex::new(client.clone()))
                .service(web::resource("/").guard(guard::Post()).to(index))
                .service(web::resource("/").guard(guard::Get()).to(index_playground))
        })
        .bind(format!("0.0.0.0:{}", self.port))?
        .run()
        .await
    }
}
