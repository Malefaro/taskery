use auth_service::client::Client;
use graphql_server::{database::postgres::PostgresDB, server::Server};

const AUTH_SERVICE_URL: &'static str = "AUTH_SERVICE_URL";
const POSTGRES_URL: &'static str = "POSTGRES_URL";
const PORT: &'static str = "GRAPHQL_SERVER_PORT";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let auth_service_url =
        std::env::var(AUTH_SERVICE_URL).expect(&format!("{} not set", AUTH_SERVICE_URL));
    let server_port = std::env::var(PORT)
        .expect(&format!("{} not set", PORT))
        .parse()
        .expect("Cannot parse string");
    let postgres_url = std::env::var(POSTGRES_URL).expect(&format!("{} not set", POSTGRES_URL));

    use tokio::runtime::Runtime;
    let rt = Runtime::new().unwrap();
    let client = rt.block_on(Client::connect(auth_service_url)).unwrap(); // actix_web before 4.x version using old tokio. So it cannot be run directly
    let server = Server::new(PostgresDB::new(&postgres_url), client, server_port);
    server.run().await
}
