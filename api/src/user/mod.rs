use axum::Json;
use model::response::Res;
use model::user::request::{CreateReq, LoginReq};
use service::user::{get_by_phone, register};
use utils::db::{init, DB};

pub async fn create(Json(req): Json<CreateReq>) -> Res<String> {
    let db = DB.get_or_init(init).await;
    let res = register(db, req).await;

    match res {
        Ok(x) => Res::ok_with_msg(x),
        Err(e) => Res::error_with_msg(500, e.to_string()),
    }
}

pub async fn login(Json(req): Json<LoginReq>) -> Res<String> {
    let db = DB.get_or_init(init).await;
    let res = get_by_phone(db, req).await;

    match res {
        Ok(x) => Res::ok_with_msg(x),
        Err(e) => Res::error_with_msg(500, e.to_string()),
    }
}
