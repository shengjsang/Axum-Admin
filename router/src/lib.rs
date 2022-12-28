use api::common::show_captcha;
use api::user::{create, login};
use axum::routing::post;
use axum::Router;

pub fn api() -> Router {
    // 嵌套path 方便我们对不同的功能进行细分和管理
    Router::new()
        .nest("/user", user_api()) //  /user/register
        .nest("/common", captcha_api())
}

fn user_api() -> Router {
    Router::new()
        .route("/register", post(create)) // 注册
        .route("/login", post(login))
}

fn captcha_api() -> Router {
    Router::new().route("/show-captcha", post(show_captcha))
}
