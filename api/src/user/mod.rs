use axum::Json;
use model::user::request::CreateReq;
use serde_json::{json, Value};
use service::user::register;
use utils::db::{init, DB};

pub async fn create(Json(req): Json<CreateReq>) -> Json<Value> {
    let db = DB.get_or_init(init).await;
    let res = register(db, req).await;

    match res {
        Ok(_x) => Json(json!({"data": 200 })),
        Err(_e) => Json(json!({"data": 404 })),
    }
}
