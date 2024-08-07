use serde::Deserialize;

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
