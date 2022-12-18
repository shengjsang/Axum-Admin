use std::net::SocketAddr;
use std::str::FromStr;
use axum::{routing::get, Router};
use tracing::{ info};
use configs::CFG;
use migration::{Migrator, MigratorTrait};
use router::create;
use utils::db::{DB, init};
use utils::log;

#[tokio::main]
async fn main() {
    // 初始化日志
    let _guard = log::init();
    info!("Starting");

    // 初始化数据库
    let db = DB.get_or_init(init).await;
    // 设置数据库迁移
    Migrator::up(db, None).await.unwrap();

    // route函数来设置路由，两个参数 一个路由路径 一个handler函数
    // handler函数就是一个异步函数来处理程序逻辑，从请求中提取解析作为参数，并返回响应，响应要实现IntoResponse
    let app = Router::new().route("/", get(create));

    let addr =SocketAddr::from_str(&CFG.server.address).unwrap();
    // 设置端口
    axum::Server::bind(&addr)
        // 服务启动
        .serve(app.into_make_service())
        .await
        .unwrap();
}
