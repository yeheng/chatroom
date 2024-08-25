use std::fmt::Debug;

use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::derive::{Display, Error};
use serde_json::json;

#[derive(Debug, Display, Error)]
pub enum CustomError {
    #[display("Validation error: {}", message)]
    ValidationError { message: String },

    #[display("Unauthorized: {}", message)]
    UnauthorizedError { message: String },

    #[display("Internal error: {}", message)]
    InternalError { message: String },
}

// 为自定义错误实现 ResponseError 以可返回 HTTP 错误
// 这里 Clion 显示错误,但实际上 build 是没有问题的
impl error::ResponseError for CustomError {
    fn status_code(&self) -> StatusCode {
        match *self {
            CustomError::ValidationError { .. } => StatusCode::UNPROCESSABLE_ENTITY,
            CustomError::UnauthorizedError { .. } => StatusCode::UNAUTHORIZED,
            CustomError::InternalError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let mut builder = HttpResponse::build(self.status_code());

        let response_message = json!({
            "error": {
                "body": self.to_string()
            }
        })
        .to_string();

        builder
            .content_type(ContentType::json())
            .body(response_message)
    }
}
