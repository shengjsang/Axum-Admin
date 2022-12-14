use api::common::jwt::protected;
use api::common::{show_captcha, test};
use api::system::info::get_info;
use api::todo;
use api::todo::{change_status, delete_task, get_all_tasks, get_task};
use api::user::{create, login};
use axum::routing::{delete, get, post};
use axum::Router;

pub fn api() -> Router {
    // 嵌套path 方便我们对不同的功能进行细分和管理
    Router::new()
        .nest("/user", user_api()) //  /user/register
        .nest("/common", common_api())
        .nest("/system", system_api())
        .nest("/todo", todo_api())
}

fn user_api() -> Router {
    Router::new()
        .route("/register", post(create)) // 注册
        .route("/login", post(login))
}

fn todo_api() -> Router {
    Router::new()
        .route("/create", post(todo::create))
        .route("/get-all", get(get_all_tasks)) // 注册
        .route("/change-status", post(change_status))
        .route("/delete-task", delete(delete_task))
        .route("/get-task", post(get_task))
}


fn common_api() -> Router {
    Router::new()
        .route("/show-captcha", post(show_captcha))
        .route("/test", get(test))
        .route("/protect", get(protected))
}

fn system_api() -> Router {
    Router::new().route("/info", get(get_info))
}
