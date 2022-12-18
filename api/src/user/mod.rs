use axum::Json;
use serde_json::{json, Value};
use service::user::register;
use utils::db::{DB, init};

pub async fn create() -> Json<Value> {
    let db = DB.get_or_init(init).await;
    let res = register(db).await;

    match res {
        Ok(x) => Json(json!({"data": 200 })),
        Err(e) => Json(json!({"data": 404 })),
    }
}
