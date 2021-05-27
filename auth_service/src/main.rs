mod proto;
mod server;
mod session_manager;

use session_manager::redis;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL not set");
    let server_port = std::env::var("SERVER_PORT")
        .expect("SERVER_PORT not set")
        .parse()
        .expect("Cannot parse string");
    let redis = redis::Redis::configure().url(redis_url).create()?;
    let server = server::Server::new(server_port, redis);
    server.run().await
}
