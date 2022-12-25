use bb8_redis::redis::cmd;
use bb8_redis::{bb8, RedisConnectionManager};

pub async fn init() {
    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool = bb8::Pool::builder().build(manager).await.unwrap();
    let mut handles = vec![];

    for _i in 0..10 {
        let pool = pool.clone();

        handles.push(tokio::spawn(async move {
            let mut conn = pool.get().await.unwrap();

            let reply: String = cmd("PING").query_async(&mut *conn).await.unwrap();

            println!("{}", reply)
        }));
    }
}
