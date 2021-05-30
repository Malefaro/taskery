mod proto;
mod server;
mod session_manager;

use session_manager::redis;

const REDIS_URL: &'static str = "REDIS_URL";
const PORT: &'static str = "AUTH_SERVICE_PORT";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let redis_url = std::env::var(REDIS_URL).expect(&format!("{} not set", REDIS_URL));
    let server_port = std::env::var(PORT)
        .expect(&format!("{} not set", PORT))
        .parse()
        .expect("Cannot parse string");
    let redis = redis::Redis::configure().url(redis_url).create()?;
    let server = server::Server::new(server_port, redis);
    server.run().await
}
