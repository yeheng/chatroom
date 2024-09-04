use std::fmt::Debug;

use actix_web::{
    error,
    http::StatusCode,
    HttpResponse,
};
use derive_more::derive::{Display, Error};
use serde::Serialize;

#[derive(Serialize)]
struct FormattedErrorResponse {
    code: u16,
    error: String,
    message: String,
}

#[derive(Debug, Display, Error)]
pub enum CustomError {
    #[display("Validation error: {}", field)]
    ValidationError { field: String },

    #[display("Unauthorized: {}", message)]
    UnauthorizedError { message: String },

    #[display("Internal error: {}", message)]
    InternalError { message: String },

    #[display("Bad request")]
    BadClientData,

    #[display("{} Not found!", message)]
    NotFound { message: String },

    #[display("{}", message)]
    DbErr {message: String}
}

impl CustomError {
    fn name(&self) -> String {
        match self {
            CustomError::ValidationError { .. } => "Validation Error".to_string(),
            CustomError::InternalError { .. } => "Internal Server Error".to_string(),
            CustomError::UnauthorizedError { .. } => "Unauthorized".to_string(),
            CustomError::BadClientData => "Bad request".to_string(),
            CustomError::NotFound { .. } => "Not found".to_string(),
            CustomError::DbErr { .. } => "DB Error".to_string(),
        }
    }
}

// 为自定义错误实现 ResponseError 以可返回 HTTP 错误
// 这里 Clion 显示错误,但实际上 build 是没有问题的
impl error::ResponseError for CustomError {
    fn status_code(&self) -> StatusCode {
        match *self {
            CustomError::InternalError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            CustomError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            CustomError::UnauthorizedError { .. } => StatusCode::UNAUTHORIZED,
            CustomError::BadClientData => StatusCode::BAD_REQUEST,
            CustomError::NotFound { .. } => StatusCode::NOT_FOUND,
            CustomError::DbErr { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let error_response = FormattedErrorResponse {
            code: self.status_code().as_u16(),
            error: self.to_string(),
            message: self.name(),
        };
        HttpResponse::build(self.status_code()).json(error_response)
    }
}
