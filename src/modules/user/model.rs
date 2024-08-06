use rbatis::impl_update;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct NewUser {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateUser {
    pub uid: u32,
    pub email: Option<String>,
    pub username: Option<String>,
    pub nickname: Option<String>,
    pub password: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub deleted: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserPayload {
    pub user: UpdateUser,
}

impl_update!(UpdateUser {}, r#""sys_user""#);
