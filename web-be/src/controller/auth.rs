use crate::structs::AuthError;
use axum::extract::Json;
use axum::http::HeaderMap;
use tracing::{debug, span, warn, Level};

use crate::structs::Claims;
use headers::HeaderValue;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
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

pub async fn register_handler(Json(payload): Json<Value>) -> Json<Value> {
    let req: RegisterAO = serde_json::from_value(payload).unwrap();
    debug!("receive params, req: {:?}", req);

    // todo
    // check code and others

    // encode pass

    // do others

    // do insert
    let create_user_r = crate::repository::mysql::UserEntity::create(
        req.nick_name.clone(),
        req.email,
        req.password,
    )
    .await;

    // create token and return
    match create_user_r {
        Ok(_) => crate::structs::global_response::new(
            crate::structs::global_response::ERROR_CODE_SUCCESS,
            "",
        ),
        Err(e) => {
            warn!("register_handler, create user err: {}", e);
            crate::structs::global_response::new(
                crate::structs::global_response::ERROR_CODE_ERROR,
                "",
            )
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct RegisterAO {
    pub nick_name: String,
    pub email: String,
    pub password: String,
    pub code: String,
}
