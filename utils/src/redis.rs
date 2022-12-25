use redis::aio::Connection;
use redis::{Client, RedisResult};

pub async fn connect() -> RedisResult<Connection> {
    let client = Client::open("redis://localhost").unwrap();
    let con = client.get_tokio_connection().await.unwrap();
    Ok(con)
}

pub async fn test(con: &mut Connection) -> RedisResult<()> {
    let _: () = redis::Cmd::new()
        .arg("SET")
        .arg("yzm")
        .arg(123456)
        .query_async(con)
        .await?;
    Ok(())
}
