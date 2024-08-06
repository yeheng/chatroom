use actix_web::{get, put, Responder, web};
use rbatis::executor::RBatisRef;

use crate::middleware::auth::{Claim, RealWorldToken};
use crate::middleware::datasource::DB;
use crate::middleware::ResponseData;
use crate::modules::user::model::{UpdateUser, UpdateUserPayload};
use crate::modules::user::UserService;
use crate::util::error::CustomError::InternalError;

#[get("/{uid}")]
pub async fn get_user_info(path: web::Path<u32>) -> Result<impl Responder, actix_web::Error> {
    let uid = path.into_inner();

    let user = UserService::select_user_by_uid(uid)
        .await
        .map_err(|e| InternalError { message: e.to_string() })?;
    Ok(ResponseData::new("user", user))
}

#[get("")]
pub async fn get_current_user(token: RealWorldToken, claims: Claim) -> Result<impl Responder, actix_web::Error> {
    let mut user = UserService::select_user_by_uid(claims.id)
        .await
        .map_err(|e| InternalError { message: e.to_string() })?;
    if let Some(ref mut u) = user {
        u.token = Some(token.token);
    }
    Ok(ResponseData::new("user", user))
}

#[put("/{uid}")]
pub async fn update_user(user: web::Json<UpdateUserPayload>, path: web::Path<u32>, claims: Claim) -> impl Responder {
    let mut user: UpdateUser = user.into_inner().user;
    let uid = path.into_inner();

    if uid != claims.id {
        return Err(InternalError { message: "Unauthorized".to_owned() });
    }

    user.uid = claims.id;
    let rb = DB.rb.rb_ref();
    let result = UpdateUser::update_by_column(rb, &user, "uid")
        .await.map_err(|e| InternalError { message: e.to_string() })?;
    if result.rows_affected < 1 {
        return Err(InternalError { message: "Update may not be successful".to_owned() });
    }
    let user = UserService::select_user_by_uid(claims.id)
        .await.map_err(|e| InternalError { message: e.to_string() })?.unwrap();

    Ok(ResponseData::new("user", user))
}
