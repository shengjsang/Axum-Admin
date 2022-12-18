use std::net::SocketAddr;
use std::str::FromStr;
use axum::{routing::get, Router};
use tracing::{ info};
use configs::CFG;
use migration::{Migrator, MigratorTrait};
use router::{api};
use utils::db::{DB, init};
use utils::log;

#[tokio::main]
async fn main() {
    // 初始化日志
    let _guard = log::init();
    info!("Starting");

    // 初始化数据库
    let _db = DB.get_or_init(init).await;

    let app = Router::new().nest("/v1", api());

    let addr =SocketAddr::from_str(&CFG.server.address).unwrap();
    // 设置端口
    axum::Server::bind(&addr)
        // 服务启动
        .serve(app.into_make_service())
        .await
        .unwrap();
}
