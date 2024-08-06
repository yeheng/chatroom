use fastdate::DateTime;
use rbatis::impl_select;

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

impl_select!(UserTable {}, r#""sys_user""#);
