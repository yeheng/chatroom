use actix_web::{http::header, post, web, HttpRequest, Responder};

use crate::middleware::ResponseData;
use crate::modules::auth::model::LoginPayload;
use crate::modules::user::model::UserVO;
use crate::modules::user::UserService;
use crate::util::{
    self,
    error::CustomError::{InternalError, UnauthorizedError},
};

#[post("/login")]
pub async fn login(request: HttpRequest, credentials: web::Json<LoginPayload>) -> impl Responder {
    let credentials = credentials.into_inner();
    let username = credentials.username.trim();
    let password = credentials.password.trim();

    let origin = util::auth_utils::get_header_value_str(&request, header::REFERER, "");
    if username.is_empty() || password.is_empty() {
        return Err(UnauthorizedError {
            realm: origin.to_owned(),
            message: "`username` and `password` is required".to_owned(),
        });
    }

    let result = UserService::select_user_by_username(username)
        .await
        .map_err(|e| InternalError {
            message: e.to_string(),
        })?;
    if result.is_none() {
        return Err(UnauthorizedError {
            realm: origin.to_owned(),
            message: "Incorrect username or password".to_owned(),
        });
    }
    let user = result.unwrap();
    let is_verified = util::auth_utils::verify_aes_password(password, user.password.as_str());
    if !is_verified {
        return Err(UnauthorizedError {
            realm: origin.to_owned(),
            message: "Incorrect username or password".to_owned(),
        });
    }
    // 密码校验通过，签发 Token
    // todo: 获取用户权限
    let token = util::auth_utils::sign_token(user.user_id, user.user_name, vec![]).unwrap();
    let user = Some(UserVO {
        user_id: user.user_id,
        username: username.to_owned(),
        nickname: user.nick_name,
        permissions: vec![],
        token: Some(token),
    });

    Ok(ResponseData::data(user))
}
