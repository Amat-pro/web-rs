pub mod global_response;
mod request_response;

use axum::{
    async_trait,
    extract::{FromRequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json, RequestPartsExt,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::warn;

pub use request_response::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalError {
    pub desc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_info: UserInfo,
    // expire_time: timestamp  ##this filed is must needed
    pub exp: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserInfo {
    pub id: String,
    pub nick_name: String,
    pub email: String,
}

#[derive(Serialize, Debug)]
pub struct Page<T: Serialize> {
    pub size: u64,
    pub current: u64,
    pub total: u64,
    pub data: T,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]

pub struct PageParam {
    pub size: u64,
    pub current: u64,
}

pub struct LimitOffset {
    pub limit: u64,
    pub offset: u64,
}

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        // Decode the user data
        let token_data = crate::utils::jwt::decode_token(bearer.token()).map_err(|e| {
            warn!("decode token err {}", e);
            AuthError::InvalidToken
        })?;

        let claims: Claims = token_data.claims;
        let now = chrono::Local::now().timestamp_millis() as u64;
        if claims.exp <= now {
            return Err(AuthError::InvalidToken);
        }

        Ok(claims)
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

impl Claims {
    pub fn new(expire_time: u64, u: UserInfo) -> Self {
        Self {
            user_info: u,
            exp: expire_time,
        }
    }
}

impl UserInfo {
    pub fn new() -> Self {
        Self {
            id: "".to_string(),
            nick_name: "".to_string(),
            email: "".to_string(),
        }
    }
}

impl GlobalError {
    pub fn new(desc: Option<String>) -> Self {
        Self { desc }
    }

    pub fn from_str(s: &str) -> Self {
        Self {
            desc: Some(s.to_string()),
        }
    }
}

pub fn new_page<T: Serialize>(size: u64, current: u64, total: u64, data: T) -> Page<T> {
    Page {
        size,
        current,
        total,
        data,
    }
}

pub fn build_limit_offset(count: u64, param: &PageParam) -> Result<LimitOffset, GlobalError> {
    if count <= 0 {
        return Err(GlobalError::new(Some(
            "build_limit_offset_opt, invalid param for count".to_string(),
        )));
    }

    if param.current <= 0 {
        return Err(GlobalError::new(Some(
            "build_limit_offset_opt, invalid param for param.current".to_string(),
        )));
    }

    if param.size <= 0 {
        return Err(GlobalError::new(Some(
            "build_limit_offset_opt, invalid param for param.size".to_string(),
        )));
    }

    let offset = (param.current - 1) * param.size;

    Ok(LimitOffset {
        limit: param.size,
        offset,
    })
}
