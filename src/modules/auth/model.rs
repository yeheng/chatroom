use fastdate::DateTime;
use rbatis::{impl_select, impl_update};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct UserTable {
    pub uid: u32,
    pub email: String,
    pub username: String,
    pub password: String,
    pub nickname: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub created_time: DateTime,
    pub updated_time: DateTime,
    pub deleted: bool,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct User {
    pub uid: u32,
    pub email: String,
    pub username: String,
    pub nickname: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub token: Option<String>,
}

impl_update!(UpdateUser {}, r#""sys_user""#);
impl_select!(UserTable {}, r#""sys_user""#);

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

#[derive(Clone, Debug, Deserialize)]
pub struct LoginCredentials {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    pub user: LoginCredentials,
}

#[derive(Debug, Deserialize)]
pub struct SignUpPayload {
    pub user: NewUser,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserPayload {
    pub user: UpdateUser,
}
