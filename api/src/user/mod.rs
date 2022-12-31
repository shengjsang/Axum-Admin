use axum::http::HeaderMap;
use axum::Json;
use model::common::jwt::AuthBody;
use model::response::Res;
use model::user::request::{CreateReq, LoginReq};
use service::user::register;
use utils::db::{init, DB};

pub async fn create(Json(req): Json<CreateReq>) -> Res<String> {
    let db = DB.get_or_init(init).await;
    let res = register(db, req).await;

    match res {
        Ok(x) => Res::ok_with_msg(x),
        Err(e) => Res::error_with_msg(500, e.to_string()),
    }
}

pub async fn login(header: HeaderMap, Json(req): Json<LoginReq>) -> Res<AuthBody> {
    let db = DB.get_or_init(init).await;
    let res = service::user::login(db, req, header).await;

    match res {
        Ok(x) => Res::ok_with_data(x),
        Err(e) => Res::error_with_msg(500, format!("{:?}", e)),
    }
}
