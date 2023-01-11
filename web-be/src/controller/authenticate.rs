use crate::structs::AuthError;
use axum::extract::Json;
use axum::http::HeaderMap;
use tracing::{debug, span, Level};

use crate::structs::Claims;
use headers::HeaderValue;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
    expire_time: u64,
}

impl AuthBody {
    fn new(access_token: String, expire_time: u64) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
            expire_time: expire_time,
        }
    }
}

pub async fn authorize(
    client_id_opt: Option<&HeaderValue>,
    client_secret_opt: Option<&HeaderValue>,
) -> Result<Json<AuthBody>, AuthError> {
    // Check if the user sent the credentials

    match (client_id_opt, client_secret_opt) {
        (Some(client_id), Some(client_secret)) => {
            // Here you can check the user credentials from a database
            if client_id != "foo" || client_secret != "bar" {
                return Err(AuthError::WrongCredentials);
            }

            // generate expire_time here
            let expire_time = 1673426526502;

            let claims = Claims::new(expire_time, "1".to_string());
            // Create the authorization token
            let token =
                crate::utils::jwt::generate_token(&claims).map_err(|_| AuthError::TokenCreation)?;

            // Send the authorized token
            Ok(Json(AuthBody::new(token, expire_time)))
        }
        _ => {
            return Err(AuthError::WrongCredentials);
        }
    }
}

pub async fn authenticate_handler(headers: HeaderMap) -> Result<Json<AuthBody>, AuthError> {
    // request
    let span = span!(Level::DEBUG, "authenticate");
    let _enter = span.enter();

    debug!("authenticating start");

    let client_id = headers.get("client_id");
    let client_secret = headers.get("client_secret");
    let r = authorize(client_id, client_secret).await;

    debug!("authenticating end");

    return r;
}
