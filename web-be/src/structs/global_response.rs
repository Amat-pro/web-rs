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
pub const ERROR_CODE_SUCCESS: ErrorCode = ErrorCode(10000 as u32, "success");
pub const ERROR_CODE_ERROR: ErrorCode = ErrorCode(10001 as u32, "error");
pub const ERROR_CODE_PARAM_INVALID: ErrorCode = ErrorCode(10002 as u32, "invalid param");

pub const ERROR_CODE_EMAIL_SEND_INVALID: ErrorCode = ErrorCode(10003 as u32, "email send invalid");
pub const ERROR_CODE_EMAIL_SEND_ERROR: ErrorCode = ErrorCode(10004 as u32, "email send error");
