use auth_service::client::Client;
use data_service::{database::postgres::PostgresDB, server::Server};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use tokio::runtime::Runtime;
    let rt = Runtime::new().unwrap();
    let client = rt
        .block_on(Client::connect("http://127.0.0.1:5051".to_string()))
        .unwrap(); // actix_web before 4.x version using old tokio. So it cannot be run directly
    let server = Server::new(
        PostgresDB::new("postgres://localhost/taskery?connect_timeout=5"),
        client,
        5050,
    );
    server.run().await
}
