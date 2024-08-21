use std::time::Duration;

use actix_http::header::HeaderName;
use actix_web::HttpRequest;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use fastdate::{DateTime, DurationFrom};
use jsonwebtoken::{Algorithm, Header, Validation};
use openssl::symm::{decrypt, Cipher};

use crate::config::{CONFIG, JWT_KEY};
use crate::middleware::auth::Claim;
use crate::util::error::CustomError;

/// 校验密码和哈希
///
/// # Arguments
///
/// * `password` - 明文密码
/// * `password_hash` - 加密后的密码哈希
///
/// # Returns
///
/// * `bool` - 返回密码是否匹配
pub fn verify_aes_password(password: &str, password_hash: &str) -> bool {
    let cipher = Cipher::aes_128_ecb();

    let decrypt = decrypt(
        cipher,
        CONFIG.key.as_bytes(),
        None,
        STANDARD.decode(password_hash).unwrap().as_slice(),
    )
        .map_err(|e| e.to_string())
        .unwrap();

    password == String::from_utf8(decrypt).unwrap()
}

/// 签发 token
///
/// # Arguments
///
/// * `id` - 用户 ID
/// * `username` - 用户名
/// * `permissions` - 用户权限列表
///
/// # Returns
///
/// * `Result<String, actix_web::Error>` - 返回签发的 token 或错误
pub fn sign_token(
    id: i64,
    username: String,
    permissions: Vec<String>,
) -> Result<String, actix_web::Error> {
    let next_week = DateTime::now() + Duration::from_day(7);
    log::debug!("token过期时间 ==> {:#?}", next_week.to_string());

    let claims = Claim {
        exp: next_week.unix_timestamp() as usize,
        iss: CONFIG.jwt.issuer.to_owned(),
        id,
        username,
        permissions,
    };
    let header = Header::new(Algorithm::EdDSA);
    let token = jsonwebtoken::encode(&header, &claims, &JWT_KEY.encoding_key).map_err(|e| {
        CustomError::InternalError {
            message: e.to_string(),
        }
    })?;

    Ok(token)
}

/// 验证 Token
///
/// # Arguments
///
/// * `token` - 需要验证的 token
/// * `host` - 请求的主机地址
///
/// # Returns
///
/// * `Result<Claim, actix_web::Error>` - 返回 token 的声明或错误
pub fn validate_token(token: &str) -> Result<Claim, actix_web::Error> {
    let mut validation = Validation::new(Algorithm::EdDSA);
    validation.validate_exp = true;
    validation.set_issuer(&[CONFIG.jwt.issuer.to_owned().as_str()]);
    // FIXME Why can't I validate the token expiry time here?
    let result =
        jsonwebtoken::decode::<Claim>(token, &JWT_KEY.decoding_key, &validation).map_err(|e| {
            CustomError::UnauthorizedError {
                message: e.to_string(),
            }
        })?;
    log::debug!("Token 的载荷 => {:#?}", &result.claims);
    Ok(result.claims)
}

/// 从 HttpRequest 请求中获取指定的请求头
///
/// # Arguments
///
/// * `request` - HTTP 请求
/// * `name` - 请求头名称
/// * `default_value` - 默认值
///
/// # Returns
///
/// * `&'a str` - 返回请求头的值或默认值
pub fn get_header_value_str<'a>(
    request: &'a HttpRequest,
    name: HeaderName,
    default_value: &'a str,
) -> &'a str {
    let result = request.headers().get(name);
    if result.is_none() {
        return default_value;
    }
    result.unwrap().to_str().unwrap()
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_sign_token() {
        let id = 1;
        let username = "admin".to_owned();
        let permissions = vec!["admin".to_owned()];
        let token = sign_token(id, username, permissions).unwrap();
        assert!(!token.is_empty());
    }

    #[test]
    fn test_validate_token() {
        let id = 1;
        let username = "admin".to_owned();
        let permissions = vec!["admin".to_owned()];
        let token = sign_token(id, username.clone(), permissions).unwrap();
        let claim = validate_token(&token).unwrap();
        assert_eq!(claim.username, username);
    }
}
