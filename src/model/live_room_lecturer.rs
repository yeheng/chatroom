//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, serde::Serialize, serde::Deserialize)]
#[sea_orm(table_name = "live_room_lecturer")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub room_id: i64,
    #[sea_orm(primary_key)]
    pub user_id: i64,
    pub indx: Option<i16>,
    pub is_vote: i16,
    pub vote_count: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
