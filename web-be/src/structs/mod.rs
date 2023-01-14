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
    pub size: usize,
    pub current: usize,
    pub total: usize,
    pub data: T,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]

pub struct PageParam {
    pub size: usize,
    pub current: usize,
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

pub fn new_page<T: Serialize>(size: usize, current: usize, total: usize, data: T) -> Page<T> {
    Page {
        size,
        current,
        total,
        data,
    }
}
