use axum::{Json, Router};
use axum::http::StatusCode;
use axum::routing::{get, get_service};
use serde_json::{json, Value};
use utils::db::{DB, init};
use api::user::{create};




pub fn api() -> Router {
    Router::new()
        .nest("/user", user_api())

}


fn user_api() -> Router {
    Router::new()
        .route("/register", get(create)) // 注册

}
