use std::time::Duration;

use actix_http::header::HeaderName;
use actix_web::HttpRequest;
use argon2::{
    Argon2,
    password_hash::{
        PasswordHash,
        PasswordHasher, PasswordVerifier, rand_core::OsRng, SaltString,
    },
};
use fastdate::{DateTime, DurationFrom};
use jsonwebtoken::{Algorithm, Header, Validation};
use log::debug;

use crate::config::config::{CONFIG, JWT_KEY};
use crate::middleware::Claim;
use crate::util::error::CustomError;

/// 获取 PHC 字符串
pub fn get_phc_string(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();
    // Hash password to PHC string ($argon2id$v=19$...)
    argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string()
}

/// 校验密码和哈希
pub fn verify_password(password: &str, password_hash: &str) -> bool {
    // NOTE: hash params from `parsed_hash` are used instead of what is configured in the `Argon2` instance.
    let parsed_hash = PasswordHash::new(password_hash).unwrap();
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
}

// 签发 token
pub fn sign_token(id: u32, email: String) -> Result<String, actix_web::Error> {
    let next_week = DateTime::now() + Duration::from_day(7);
    debug!("过期时间 ==> {:#?}", next_week.to_string());

    let claims = Claim {
        exp: next_week.unix_timestamp() as usize,
        iss: CONFIG.jwt.issuer.to_owned(),
        id,
        email,
    };
    let header = Header::new(Algorithm::EdDSA);
    let token = jsonwebtoken::encode(&header, &claims, &JWT_KEY.encoding_key)
        .map_err(|e| CustomError::InternalError { message: e.to_string() })?;

    Ok(token)
}

// 验证 Token
pub fn validate_token(token: &str, host: &str) -> Result<Claim, actix_web::Error> {
    let mut validation = Validation::new(Algorithm::EdDSA);
    validation.validate_exp = true;
    validation.set_issuer(&[CONFIG.jwt.issuer.to_owned().as_str()]);
    // FIXME Why can't I validate the token expiry time here?
    let result = jsonwebtoken::decode::<Claim>(token, &JWT_KEY.decoding_key, &validation)
        .map_err(|e| CustomError::UnauthorizedError {
            realm: host.to_owned(),
            message: e.to_string(),
        })?;
    debug!("Token 的载荷 => {:#?}", &result.claims);

    Ok(result.claims)
}

// 从 HttpRequest 请求中获取指定的请求头
pub fn get_header_value_str<'a>(request: &'a HttpRequest, name: HeaderName, default_value: &'a str) -> &'a str {
    let result = request.headers().get(name);
    if result.is_none() {
        return default_value;
    }
    result.unwrap().to_str().unwrap()
}