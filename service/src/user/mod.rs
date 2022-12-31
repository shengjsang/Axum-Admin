use anyhow::Result;
use headers::HeaderMap;
use model::entity::user;
use model::user::request::{CreateReq, LoginReq};
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, QueryFilter, QueryOrder};
use user::Entity as User;

use crate::common::jwt::authorize;
use model::common::jwt::{AuthBody, AuthError, AuthPayload};
use sea_orm::EntityTrait;
use utils::encrypt::encrypt_password;

pub async fn register(db: &DatabaseConnection, req: CreateReq) -> Result<String> {
    let id = scru128::new_string();
    let (salt, password) = encrypt_password(req.password);
    let user = user::ActiveModel {
        id: Set(id),
        username: Set(req.username),
        phone: Set(req.phone),
        email: Set(req.email),
        password: Set(password),
        salt: Set(salt),
    };

    User::insert(user).exec(db).await?;

    Ok("用户注册成功".to_string())
}

pub async fn login(
    db: &DatabaseConnection,
    req: LoginReq,
    _header: HeaderMap,
) -> Result<AuthBody, AuthError> {
    let account = req.account.as_str();

    let user = User::find()
        .filter(
            Condition::any()
                .add(user::Column::Phone.eq(account))
                .add(user::Column::Email.eq(account))
                .add(user::Column::Username.eq(account)),
        )
        .order_by_asc(user::Column::Username)
        .one(db)
        .await
        .unwrap();

    return if user.is_some() {
        let payload = AuthPayload {
            account: account.to_string(),
        };
        let res = authorize(payload).await;
        res
    } else {
        Err(AuthError::Unknown)
    };
}
