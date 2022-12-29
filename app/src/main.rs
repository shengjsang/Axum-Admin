use axum::Router;
use configs::CFG;
use router::api;
use std::net::SocketAddr;
use std::str::FromStr;
use tracing::info;
use utils::log;

#[tokio::main]
async fn main() {
    service::system::info::get();
    // 初始化日志
    let _guard = log::init();
    info!("Starting");

    let app = Router::new().nest("/v1", api());

    let addr = SocketAddr::from_str(&CFG.server.address).unwrap();
    // 设置端口
    axum::Server::bind(&addr)
        // 服务启动
        .serve(app.into_make_service())
        .await
        .unwrap();
}
