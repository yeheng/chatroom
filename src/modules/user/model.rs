use serde::{Deserialize, Serialize};

use crate::model::sys_user;

#[derive(Clone, Debug, Deserialize)]
pub struct NewUser {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserPayload {
    pub user: sys_user::Model,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserVO {
    pub user_id: i64,
    pub username: String,
    pub nickname: String,
    pub permissions: Vec<String>,
    pub token: Option<String>,
}
