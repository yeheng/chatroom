use actix_web::{http::header::ContentType, HttpRequest, HttpResponse, Responder};
use fancy_regex::Regex;
use serde::Serialize;
use serde_json::json;

pub mod auth;
pub mod datasource;
pub mod redis;

// 统一响应结构体
#[derive(Debug, Serialize)]
pub struct ResponseData {
    pub data: String,
    pub code: u16,
}

impl ResponseData {
    pub fn new<T>(property_name: &str, data: T) -> Self
    where
        T: serde::Serialize,
    {
        Self {
            data: json!({property_name: data}).to_string(),
            code: 200,
        }
    }

    pub fn data<T>(data: T) -> Self
    where
        T: serde::Serialize,
    {
        Self {
            data: json!(data).to_string(),
            code: 200,
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
            .body(self.data)
    }
}