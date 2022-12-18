use axum::Json;
use serde_json::{json, Value};
use utils::db::{DB, init};
use service::create_user;

pub async fn create() -> Json<Value> {
    let db = DB.get_or_init(init).await;
    let res = create_user(db).await;

    match res {
        Ok(x) => Json(json!({"data": 200 })),
        Err(e) => Json(json!({"data": 404 })),
    }
}
