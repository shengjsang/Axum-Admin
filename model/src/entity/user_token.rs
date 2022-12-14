//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.4

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user_token")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub token_id: String,
    pub token_type: String,
    pub iat: DateTime,
    pub exp: DateTime,
    #[sea_orm(column_type = "Text")]
    pub token: String,
    pub account: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::Account",
        to = "super::user::Column::Username",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
