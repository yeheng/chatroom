use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub user_id: u32,
    pub email: String,
    pub user_name: String,
    pub nick_name: Option<String>,
    pub token: Option<String>,
}
