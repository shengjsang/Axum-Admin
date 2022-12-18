use sea_orm::ActiveValue::Set;
use sea_orm::{DatabaseConnection, TransactionTrait};
use model::entity::user;
use anyhow::Result;
use axum::Json;
use sea_orm::EntityTrait;
use serde_json::{json, Value};
use utils::db::{DB, init};


pub async fn register(db: &DatabaseConnection) -> Result<String> {
    let user = user::ActiveModel {
        id: Set("1".to_string()),
        username: Set("jack".to_string()),
        phone: Set("+861234567809".to_string()),
        email: Set("123456@qq.cim".to_string()),
        password: Set("123456".to_string()),
    };

    user::Entity::insert(user).exec(db).await?;

    Ok("用户添加成功".to_string())
}
