use actix_web::{post, web, Responder};

use crate::middleware::ResponseData;
use crate::modules::auth::model::LoginPayload;
use crate::modules::auth::SERVICE;
use crate::AppState;

use crate::util::error::CustomError::UnauthorizedError;

#[post("/login")]
pub async fn login(
    data: web::Data<AppState>,
    credentials: web::Json<LoginPayload>,
) -> impl Responder {
    let credentials = credentials.into_inner();

    let user = SERVICE.login(data, credentials).await;

    match user {
        Ok(Some(user)) => Ok(ResponseData::data(user)),
        Ok(None) => Err(UnauthorizedError {
            message: user.unwrap_err().to_string(),
        }),
        Err(e) => Err(e),
    }
}
