use redis::{AsyncCommands, RedisError};
#[derive(Default, Debug)]
pub struct RedisBuilder {
    url: Option<String>,
    port: Option<i32>,
    host: Option<String>,
    username: Option<String>,
    password: Option<String>,
    db: Option<String>,
}

impl RedisBuilder {
    pub fn url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }
    pub fn host(mut self, host: String) -> Self {
        self.host = Some(host);
        self
    }
    pub fn username(mut self, username: String) -> Self {
        self.username = Some(username);
        self
    }
    pub fn password(mut self, password: String) -> Self {
        self.password = Some(password);
        self
    }
    pub fn db(mut self, db: String) -> Self {
        self.db = Some(db);
        self
    }
    pub fn port(mut self, port: i32) -> Self {
        self.port = Some(port);
        self
    }
    pub fn create(self) -> Result<Redis, RedisError> {
        let url: String;
        if let Some(config_url) = self.url.clone() {
            url = config_url;
        } else {
            // use match instead
            url = format!(
                "redis://{}{}@{}:{}/{}",
                self.username.clone().unwrap_or("".to_string()),
                self.password
                    .clone()
                    .map(|x| format!(":{}", x))
                    .unwrap_or("".to_string()),
                self.host.clone().unwrap_or("0.0.0.0".to_string()),
                self.port.unwrap_or(6379),
                self.db.clone().unwrap_or("".to_string()),
            );
        }
        let redis = Redis {
            client: redis::Client::open(url).expect(&format!(
                "cannot create redis client from builder: {:?}",
                self
            )),
        };
        // check connection
        let _ = redis.client.get_connection()?;
        Ok(redis)
    }
}
pub struct Redis {
    pub client: redis::Client,
}

impl Redis {
    pub fn configure() -> RedisBuilder {
        RedisBuilder::default()
    }
}

#[async_trait::async_trait]
impl super::SessionManager for Redis {
    async fn get_user_id_by_token(&self, token: &str) -> Result<i32, Box<dyn std::error::Error>> {
        let mut c = self.client.get_async_connection().await?;
        Ok(c.get(token).await?)
    }

    async fn set_token_for_user(
        &self,
        token: &str,
        user_id: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut c = self.client.get_async_connection().await?;
        let _: () = c.set(token, user_id).await?;
        Ok(())
        // Ok(c.set(token, user_id).await?.map(|_: i32|()))
    }

    async fn delete_token(&self, token: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut c = self.client.get_async_connection().await?;
        Ok(c.del(token).await?)
    }
}
