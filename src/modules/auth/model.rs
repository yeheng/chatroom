use serde::Deserialize;

use crate::modules::user::model::NewUser;

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
