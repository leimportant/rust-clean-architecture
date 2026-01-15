use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    pub errors: Option<Value>,
}

pub fn ok<T: Serialize>(data: T, message: impl Into<String>) -> ApiResponse<T> {
    ApiResponse {
        success: true,
        data: Some(data),
        message: Some(message.into()),
        errors: None,
    }
}

pub fn created<T: Serialize>(data: T, message: impl Into<String>) -> ApiResponse<T> {
    ApiResponse {
        success: true,
        data: Some(data),
        message: Some(message.into()),
        errors: None,
    }
}

pub fn error(message: impl Into<String>) -> ApiResponse<()> {
    ApiResponse {
        success: false,
        data: None,
        message: Some(message.into()),
        errors: None,
    }
}

pub fn validation_error<E: Serialize>(errors: E) -> ApiResponse<()> {
    ApiResponse {
        success: false,
        data: None,
        message: Some("validation error".into()),
        errors: Some(serde_json::to_value(errors).unwrap()),
    }
}
