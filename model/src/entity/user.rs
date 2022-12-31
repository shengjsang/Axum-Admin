//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.4

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(unique)]
    pub username: String,
    pub password: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(unique)]
    pub phone: String,
    #[sea_orm(unique)]
    pub email: String,
    pub salt: String,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
