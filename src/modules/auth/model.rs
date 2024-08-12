use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}
