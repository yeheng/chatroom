use actix_web::web;

use crate::{
    modules::user::{model::UserVO, UserService},
    util::{
        auth_utils::{sign_token, verify_aes_password},
        error::CustomError,
    },
    AppState,
};

use super::model::LoginPayload;

pub struct AuthService;

impl AuthService {
    pub async fn login(
        &self,
        data: web::Data<AppState>,
        credentials: LoginPayload,
    ) -> Result<Option<UserVO>, CustomError> {
        let username = credentials.username.trim();
        let passwd_raw = credentials.password.trim();
        let result = UserService::select_user_by_username(data, username)
            .await?;
        if result.is_none() {
            return Err(CustomError::NotFound {
                message: format!("User {:?} not found", username).into(),
            });
        }
        let user = result.unwrap();
        let is_verified = verify_aes_password(passwd_raw, user.password.unwrap().as_str());
        if !is_verified {
            return Err(CustomError::UnauthorizedError {
                message: "Incorrect username or password".into(),
            });
        }
        // 密码校验通过,签发 Token
        // todo: 获取用户权限
        let token = sign_token(user.user_id, user.user_name, vec![]).unwrap();
        let user = Some(UserVO {
            user_id: user.user_id,
            username: username.to_owned(),
            nickname: user.nick_name,
            permissions: vec![],
            token: Some(token),
        });

        Ok(user)
    }
}
