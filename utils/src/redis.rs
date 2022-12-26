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

pub async fn set(con: &mut Connection, key: &str, value: &str, expire: u32) -> RedisResult<()> {
    redis::Cmd::new()
        .arg("Set")
        .arg(key)
        .arg(value)
        .query_async(con)
        .await?;

    redis::Cmd::new()
        .arg("Expire")
        .arg(key)
        .arg(expire)
        .query_async(con)
        .await?;

    Ok(())
}
