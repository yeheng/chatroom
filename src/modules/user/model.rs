use serde::{Deserialize, Serialize};

use crate::models::SysUser;

#[derive(Clone, Debug, Deserialize)]
pub struct NewUser {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserPayload {
    pub user: SysUser,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserVO {
    pub user_id: u32,
    pub username: String,
    pub nickname: String,
    pub permissions: Vec<String>,
    pub token: Option<String>,
}
