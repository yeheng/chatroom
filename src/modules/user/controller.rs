use actix_web::{get, put, web, Responder};

use crate::middleware::auth::{Claim, RealWorldToken};
use crate::middleware::ResponseData;
use crate::model::sys_user;
use crate::modules::user::model::{UpdateUserPayload, UserVO};
use crate::modules::user::UserService;
use crate::util::error::CustomError::{InternalError, NotFound};
use crate::AppState;

#[get("/{uid}")]
pub async fn get_user_info(path: web::Path<i64>, data: web::Data<AppState>) -> impl Responder {
    let uid = path.into_inner();
    let user = UserService::select_user_by_uid(data, uid)
        .await
        .map_err(|e| InternalError {
            message: e.to_string(),
        });
    match user {
        Ok(Some(u)) => Ok(ResponseData::data(u)),
        Ok(None) => Err(NotFound {
            message: format!("User with id:{}", uid).to_owned(),
        }),
        Err(e) => Err(e),
    }
}

#[get("")]
pub async fn get_current_user(
    token: RealWorldToken,
    data: web::Data<AppState>,
    claims: Claim,
) -> impl Responder {
    let mut user = UserService::select_user_by_uid(data, claims.id)
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
        Err(InternalError {
            message: "Unauthorized".to_owned(),
        })
    }
}

#[put("/{uid}")]
pub async fn update_user(
    user: web::Json<UpdateUserPayload>,
    data: web::Data<AppState>,
    path: web::Path<i64>,
    claims: Claim,
) -> impl Responder {
    let mut user: sys_user::Model = user.into_inner().user;
    let uid = path.into_inner();

    if uid != claims.id {
        return Err(InternalError {
            message: "Unauthorized".to_owned(),
        });
    }

    user.user_id = claims.id;

    let result = UserService::update_user(data.clone(), &user)
        .await
        .map_err(|e| InternalError {
            message: e.to_string(),
        })?;
    if result < 1 {
        return Err(InternalError {
            message: "Update may not be successful".to_owned(),
        });
    }
    let user = UserService::select_user_by_uid(data.clone(), claims.id)
        .await
        .map_err(|e| InternalError {
            message: e.to_string(),
        })?
        .unwrap();

    Ok(ResponseData::data(user))
}
