#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn case() {
        // super bad integration test to make sure everything works.
        // Do not use as real test (just for manually run)
        use auth_service::*;
        let redis_url = "redis://localhost/".to_string();
        let server_port = 5000;
        let redis = session_manager::redis::Redis::configure()
            .url(redis_url)
            .create()
            .unwrap();
        let server = server::Server::new(server_port, redis);
        let server_fut = server.run();
        tokio::spawn(async move {
            server_fut.await.unwrap();
        });
        // wait for server startup (bad practice ^_^)
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        let user_id = 123;
        let mut client = client::Client::connect(format!("http://127.0.0.1:{}", server_port))
            .await
            .unwrap();
        let token = client.sign_in(user_id).await.unwrap();
        let checked = client.check_auth(token.clone()).await.unwrap();
        println!("user_id:{} | token:{} |checked:{}", user_id, token, checked);
        assert_eq!(user_id, checked);
        let _ = client.logout(token.clone()).await.unwrap();
        let del = client.check_auth(token.clone()).await;
        println!("{:?}", del);
        assert!(del.is_err());
    }
}
