use std::time::Duration;
use sea_orm::{entity::prelude::DatabaseConnection, ConnectOptions, Database};
use tokio::sync::OnceCell;
use configs::CFG;

//  异步初始化数据库
pub static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn init() -> DatabaseConnection {
    let mut opt = ConnectOptions::new(CFG.database.url.to_owned());
    opt.max_connections(1000)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(10))
        .sqlx_logging(true);
    let db = Database::connect(opt).await.expect("数据库连接失败");
    db
}
