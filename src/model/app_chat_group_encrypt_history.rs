//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, serde::Serialize, serde::Deserialize)]
#[sea_orm(table_name = "app_chat_group_encrypt_history")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub group_ids: String,
    pub group_names: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub content: String,
    #[sea_orm(column_type = "Text")]
    pub encrypt_content: String,
    pub decrypt_time: DateTime,
    pub decrypt_pwd: Option<String>,
    pub decrypt_status: Option<i16>,
    pub from_user_id: i64,
    pub from_user_name: String,
    pub from_nick_name: String,
    pub from_user_type: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub from_user_avatar: Option<String>,
    pub from_level_id: Option<i64>,
    pub from_level_name: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub from_level_avatar: Option<String>,
    pub from_dept_id: Option<i64>,
    pub from_dept_name: Option<String>,
    pub from_dept_indx: Option<i16>,
    pub from_lecturer_name: Option<String>,
    pub from_lecturer_level: Option<i16>,
    pub from_lecturer_card_no: Option<String>,
    pub created_by: i64,
    pub created_time: DateTime,
    pub updated_by: i64,
    pub updated_time: DateTime,
    pub status: i16,
    pub del_flag: i16,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
