use axum::{
    routing::get,
    Router,
};


#[tokio::main]
async fn main() {
    // route函数来设置路由，两个参数 一个路由路径 一个handler函数
    // handler函数就是一个异步函数来处理程序逻辑，从请求中提取解析作为参数，并返回响应，响应要实现IntoResponse
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));


    // 设置端口
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        // 服务启动
        .serve(app.into_make_service())
        .await
        .unwrap();
}
