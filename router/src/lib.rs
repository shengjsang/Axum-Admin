use api::user::create;
use axum::routing::get;
use axum::Router;

pub fn api() -> Router {
    Router::new().nest("/user", user_api())
}

fn user_api() -> Router {
    Router::new().route("/register", get(create)) // 注册
}
