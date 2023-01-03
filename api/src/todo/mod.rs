use axum::Json;
use model::entity::prelude::Todo;
use model::entity::todo::Model;
use model::response::Res;
use model::todo::request::CreateReq;
use service::todo::create_task;
use utils::db::{init, DB};

pub async fn create(Json(req): Json<CreateReq>) -> Res<String> {
    let db = DB.get_or_init(init).await;
    let res = create_task(db, req).await;

    match res {
        Ok(x) => Res::ok_with_msg(x),
        Err(e) => Res::error_with_msg(500, e.to_string()),
    }
}

pub async fn get_all_tasks() -> Res<Vec<Model>> {
    let db = DB.get_or_init(init).await;
    let res = service::todo::get_all_tasks(db).await;

    match res {
        Ok(x) => Res::ok_with_data(x),
        Err(e) => Res::error_with_msg(500, e.to_string()),
    }
}
