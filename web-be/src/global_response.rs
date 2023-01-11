use axum::extract::Json;
use serde::Serialize;
use serde_json::{json, Value};

pub fn new<T>(error_code: ErrorCode, data: T) -> Json<Value>
where
    T: Serialize,
{
    Json(json!({
        "code":error_code.get_code(),
        "message":error_code.get_message(),
        "data":data,
    }))
}

pub struct ErrorCode(u32, &'static str);

impl ErrorCode {
    pub fn get_code(&self) -> u32 {
        self.0.clone()
    }

    pub fn get_message(&self) -> &str {
        self.1.clone()
    }
}

// definite error code like follows
pub const ERROR_CODE_DEFAULT: ErrorCode = ErrorCode(10000 as u32, "success");
