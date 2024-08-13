use actix_http::header;
use actix_web::dev::ServiceRequest;
use actix_web::error::ErrorBadRequest;
use actix_web::{dev, FromRequest, HttpRequest};
use chrono::{Local, Timelike};
use futures_util::future::{self, LocalBoxFuture};

use crate::util;
use crate::util::error::CustomError::UnauthorizedError;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Claim {
    // 必要,过期时间,UTC 时间戳
    pub exp: usize,
    // 可选，签发人
    pub iss: String,
    pub id: i64,
    pub username: String,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RealWorldToken {
    pub scheme: String,
    pub token: String,
}

///Whether the interface is in the whitelist
pub async fn validator(
    req: ServiceRequest,
    credentials: actix_web::Result<RealWorldToken>,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    let origin = util::auth_utils::get_header_value_str(req.request(), header::REFERER, "");
    let token = match credentials {
        Err(e) => {
            log::debug!("解析token出错 ==> {:#?}", e);
            return Err((
                UnauthorizedError {
                    realm: origin.to_owned(),
                    message: e.to_string(),
                }
                .into(),
                req,
            ));
        }
        Ok(token) => token,
    };
    log::debug!("校验Token => {:#?}", &token);
    let RealWorldToken { token, scheme } = token;
    match scheme.as_str() {
        "Bearer" => {}
        _ => {
            return Err((
                UnauthorizedError {
                    realm: origin.to_owned(),
                    message: "Invalid header value".to_owned(),
                }
                .into(),
                req,
            ))
        }
    };

    let result = util::auth_utils::validate_token(&token, origin);
    let now = Local::now().nanosecond() as usize;
    match result {
        Ok(claims) if now < claims.exp => Ok(req),
        Ok(_) => {
            log::warn!("Token 已过期！");
            let error = UnauthorizedError {
                realm: origin.to_owned(),
                message: "Token expired".to_owned(),
            };
            Err((error.into(), req))
        }
        Err(err) => Err((err, req)),
    }
}

// 为 Token 实现 actix-web 提取器
impl FromRequest for RealWorldToken {
    type Error = actix_web::Error;
    type Future = future::Ready<actix_web::Result<Self, Self::Error>>;

    fn from_request(request: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let authorization = request.headers().get(header::AUTHORIZATION);
        if authorization.is_none() {
            return future::err(ErrorBadRequest("Authentication required!"));
        }
        let mut parts = authorization.unwrap().to_str().unwrap().splitn(2, ' ');

        let scheme = parts.next().map(|s| s.to_owned());
        if scheme.is_none() || scheme.as_ref().is_some_and(|s| s.is_empty()) {
            return future::err(ErrorBadRequest("Missing authorization scheme"));
        }

        let token = parts.next().map(|s| s.to_owned());
        if token.is_none() || token.as_ref().is_some_and(|s| s.is_empty()) {
            return future::err(ErrorBadRequest("Invalid header value"));
        }

        let scheme = scheme.unwrap();
        let token = token.unwrap();
        future::ok(RealWorldToken { scheme, token })
    }
}

// 为 Claims 实现 actix-web 提取器
impl FromRequest for Claim {
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, actix_web::Result<Self, Self::Error>>;

    fn from_request(request: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let request = request.to_owned();
        Box::pin(async move {
            let RealWorldToken { token, .. } = RealWorldToken::extract(&request).await?;
            let origin = util::auth_utils::get_header_value_str(&request, header::REFERER, "");

            util::auth_utils::validate_token(&token, origin)
        })
    }
}
