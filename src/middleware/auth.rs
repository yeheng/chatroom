use actix_http::header;
use actix_web::dev::ServiceRequest;
use actix_web::error::ErrorBadRequest;
use actix_web::{dev, FromRequest, HttpRequest};
use chrono::{Local, Timelike};
use futures_util::future::{self, LocalBoxFuture};

use crate::util;
use crate::util::error::CustomError::UnauthorizedError;

// 定义错误信息常量
const AUTH_REQUIRED: &str = "Authentication required!";
const MISSING_SCHEME: &str = "Missing authorization scheme";
const INVALID_HEADER: &str = "Invalid header value";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Claim {
    // 必要,过期时间,UTC 时间戳
    pub exp: usize,
    // 可选,签发人
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

///接口是否在白名单中
pub async fn validator(
    req: ServiceRequest,
    credentials: actix_web::Result<RealWorldToken>,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    // 获取请求头中的 Referer 值
    let token = match credentials {
        Err(e) => {
            log::debug!("解析token出错 ==> {:#?}", e);
            return Err((
                UnauthorizedError {
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
                    message: "Invalid header value".to_owned(),
                }
                .into(),
                req,
            ))
        }
    };

    // 验证 Token
    let result = util::auth_utils::validate_token(&token);
    let now = Local::now().nanosecond() as usize;
    match result {
        Ok(claims) if now < claims.exp => Ok(req),
        Ok(_) => {
            log::warn!("Token 已过期！");
            let error = UnauthorizedError {
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
        // 获取请求头中的 Authorization 值
        let authorization = request.headers().get(header::AUTHORIZATION);

        match authorization {
            Some(auth_header) => {
                match auth_header.to_str() {
                    Ok(header_value) => {
                        let mut parts = header_value.splitn(2, ' ');

                        let scheme = parts.next().map(|s| s.to_owned());
                        if scheme.is_none() || scheme.as_ref().unwrap_or(&String::new()).is_empty()
                        {
                            return future::err(ErrorBadRequest(MISSING_SCHEME));
                        }

                        let token = parts.next().map(|s| s.to_owned());
                        if token.is_none() || token.as_ref().unwrap_or(&String::new()).is_empty() {
                            return future::err(ErrorBadRequest(INVALID_HEADER));
                        }

                        // 验证 scheme 是否为 Bearer
                        if scheme.as_deref() != Some("Bearer") {
                            return future::err(ErrorBadRequest(MISSING_SCHEME));
                        }

                        let scheme = scheme.unwrap();
                        let token = token.unwrap();
                        future::ok(RealWorldToken { scheme, token })
                    }
                    Err(_) => {
                        // 尝试从请求参数中获取 token
                        let query_params = request.uri().query();
                        if let Some(query) = query_params {
                            let params: Vec<&str> = query.split('&').collect();
                            for param in params {
                                let parts: Vec<&str> = param.split('=').collect();
                                if parts.len() == 2 && parts[0] == "token" {
                                    let token = parts[1];
                                    if token.is_empty() {
                                        return future::err(ErrorBadRequest(INVALID_HEADER));
                                    }
                                    return future::ok(RealWorldToken {
                                        scheme: "Bearer".to_owned(),
                                        token: token.to_owned(),
                                    });
                                }
                            }
                        }
                        future::err(ErrorBadRequest(INVALID_HEADER))
                    }
                }
            }
            None => {
                // 尝试从请求参数中获取 token
                let query_params = request.uri().query();
                if let Some(query) = query_params {
                    let params: Vec<&str> = query.split('&').collect();
                    for param in params {
                        let parts: Vec<&str> = param.split('=').collect();
                        if parts.len() == 2 && parts[0] == "token" {
                            let token = parts[1];
                            if token.is_empty() {
                                return future::err(ErrorBadRequest(INVALID_HEADER));
                            }
                            return future::ok(RealWorldToken {
                                scheme: "Bearer".to_owned(),
                                token: token.to_owned(),
                            });
                        }
                    }
                }
                future::err(ErrorBadRequest(AUTH_REQUIRED))
            }
        }
    }
}

// 为 Claims 实现 actix-web 提取器
impl FromRequest for Claim {
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, actix_web::Result<Self, Self::Error>>;

    fn from_request(request: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let request = request.to_owned();
        Box::pin(async move {
            // 提取 Token
            let RealWorldToken { token, .. } = RealWorldToken::extract(&request).await?;

            // 验证 Token 并返回 Claims
            util::auth_utils::validate_token(&token)
        })
    }
}
