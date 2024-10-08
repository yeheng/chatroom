//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, serde::Serialize, serde::Deserialize)]
#[sea_orm(table_name = "live_mobile_nav_bar")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub theme_id: i64,
    pub name: String,
    pub code: String,
    pub indx: i16,
    pub disable: i16,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
