use axum::Json;
use serde_json::{json, Value};
use service::user::register;
use utils::db::{init, DB};

pub async fn create() -> Json<Value> {
    let db = DB.get_or_init(init).await;
    let res = register(db).await;

    match res {
        Ok(_x) => Json(json!({"data": 200 })),
        Err(_e) => Json(json!({"data": 404 })),
    }
}
