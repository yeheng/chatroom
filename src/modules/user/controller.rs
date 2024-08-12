use actix_web::{get, put, web, Responder};

use crate::middleware::auth::{Claim, RealWorldToken};
use crate::middleware::ResponseData;
use crate::models::SysUser;
use crate::modules::user::model::{UpdateUserPayload, UserVO};
use crate::modules::user::UserService;
use crate::util::error::CustomError::InternalError;

#[get("/{uid}")]
pub async fn get_user_info(path: web::Path<u32>, claims: Claim) -> impl Responder {
    let uid = path.into_inner();
    if uid != claims.id {
        return Err(InternalError {
            message: "Unauthorized".to_owned(),
        });
    }

    let user = UserService::select_user_by_uid(uid)
        .await
        .map_err(|e| InternalError {
            message: e.to_string(),
        })?;
    Ok(ResponseData::new("user", user))
}

#[get("")]
pub async fn get_current_user(token: RealWorldToken, claims: Claim) -> impl Responder {
    let mut user = UserService::select_user_by_uid(claims.id)
        .await
        .map_err(|e| InternalError {
            message: e.to_string(),
        })?;
    if let Some(ref mut u) = user {
        let data = UserVO {
            user_id: u.user_id,
            username: u.user_name.clone(),
            nickname: u.nick_name.clone(),
            permissions: vec![],
            token: Some(token.token),
        };
        Ok(ResponseData::data(data))
    } else {
        return Err(InternalError {
            message: "Unauthorized".to_owned(),
        });
    }
}

#[put("/{uid}")]
pub async fn update_user(
    user: web::Json<UpdateUserPayload>,
    path: web::Path<u32>,
    claims: Claim,
) -> impl Responder {
    let mut user: SysUser = user.into_inner().user;
    let uid = path.into_inner();

    if uid != claims.id {
        return Err(InternalError {
            message: "Unauthorized".to_owned(),
        });
    }

    user.user_id = claims.id;

    let result = UserService::update_user(&user)
        .await
        .map_err(|e| InternalError {
            message: e.to_string(),
        })?;
    if result.rows_affected < 1 {
        return Err(InternalError {
            message: "Update may not be successful".to_owned(),
        });
    }
    let user = UserService::select_user_by_uid(claims.id)
        .await
        .map_err(|e| InternalError {
            message: e.to_string(),
        })?
        .unwrap();

    Ok(ResponseData::data(user))
}
