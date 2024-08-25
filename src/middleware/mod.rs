use actix_http::body::BoxBody;
use actix_http::{header, StatusCode};
use actix_web::http::header::ContentType;
use actix_web::middleware::ErrorHandlerResponse;
use actix_web::{dev, HttpRequest, HttpResponse, Responder};
use fancy_regex::Regex;
use serde_json::json;

pub mod auth;
pub mod datasource;
pub mod redis;
pub mod kafka;

// 统一响应结构体
#[derive(Debug, serde::Serialize)]
pub struct ResponseData {
    pub body: String,
}

impl ResponseData {
    pub fn new<T>(property_name: &str, data: T) -> Self
    where
        T: serde::Serialize,
    {
        Self {
            body: json!({property_name: data}).to_string(),
        }
    }

    pub fn data<T>(data: T) -> Self
    where
        T: serde::Serialize,
    {
        Self {
            body: json!(data).to_string(),
        }
    }

    pub fn same<T>(data: T) -> Self
    where
        T: serde::Serialize,
    {
        let type_name = std::any::type_name::<T>();
        let list: Vec<&str> = type_name.rsplitn(2, "::").collect();
        let type_name = list[0].replace('>', "").to_lowercase();
        let regex = Regex::new(r"^(?<=<).+?(?=>)$").unwrap();
        let result = regex.find(&type_name);
        let type_name = match result {
            Err(_) => &type_name,
            Ok(option) => match option {
                None => &type_name,
                Some(m) => m.as_str(),
            },
        };
        Self::new(type_name, data)
    }
}

// 为返回体实现 actix Responder
impl Responder for ResponseData {
    type Body = actix_http::body::BoxBody;

    fn respond_to(self, _request: &HttpRequest) -> HttpResponse<Self::Body> {
        // Create HttpResponse and set Content Type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(self.body)
    }
}

pub fn format_response<B>(
    mut response: dev::ServiceResponse<B>,
) -> actix_web::Result<ErrorHandlerResponse<B>> {
    // 重写请求头的 content-type
    response.response_mut().headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json; charset=utf-8"),
    );
    // 重写 Http StatusCode 为 422
    response.response_mut().head_mut().status = StatusCode::UNPROCESSABLE_ENTITY;
    // 获取框架的错误信息
    let error_message: String = match response.response().error() {
        Some(e) => e.to_string(),
        None => String::from("Unknown Error"),
    };
    // 格式化响应体为要求的返回格式
    let body = json!({
        "error": {
            "body": [error_message]
        }
    })
    .to_string();
    let new_response = response.map_body(move |_head, _body| BoxBody::new(body));

    Ok(ErrorHandlerResponse::Response(
        new_response.map_into_right_body(),
    ))
}
