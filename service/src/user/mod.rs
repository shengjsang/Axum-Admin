use anyhow::Result;
use model::entity::user;

use model::user::request::CreateReq;
use sea_orm::ActiveValue::Set;
use sea_orm::DatabaseConnection;

use sea_orm::EntityTrait;

pub async fn register(db: &DatabaseConnection, req: CreateReq) -> Result<String> {
    let id = scru128::new_string();
    let user = user::ActiveModel {
        id: Set(id),
        username: Set(req.username),
        phone: Set(req.phone),
        email: Set(req.email),
        password: Set(req.password),
    };

    user::Entity::insert(user).exec(db).await?;

    Ok("用户注册成功".to_string())
}
