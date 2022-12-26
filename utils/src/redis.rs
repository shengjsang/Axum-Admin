use redis::aio::Connection;
use redis::{Client, RedisResult};
use tracing::info;

pub async fn connect() -> RedisResult<Connection> {
    let client = Client::open("redis://localhost")?;
    let mut con = client.get_tokio_connection().await?;
    let res: String = redis::Cmd::new()
        .arg("Ping")
        .query_async(&mut con)
        .await
        .unwrap();
    info!("Redis Ping {}", res);
    Ok(con)
}
