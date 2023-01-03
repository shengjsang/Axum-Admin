use anyhow::Result;
use chrono::Local;
use model::entity::todo;
use model::todo::request::CreateReq;
use sea_orm::ActiveValue::Set;
use sea_orm::{DatabaseConnection, EntityTrait};
use std::process::id;
use todo::Entity as Todo;

pub async fn create_task(db: &DatabaseConnection, req: CreateReq) -> Result<String> {
    let task = todo::ActiveModel {
        title: Set(Some(req.title)),
        content: Set(Some(req.content)),
        status: Set(Some(false)),
        created_at: Set(Some(Local::now().naive_local())),
        updated_at: Set(None),
        id: Default::default(),
    };

    Todo::insert(task).exec(db).await?;

    Ok("创建成功".to_string())
}
