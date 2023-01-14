use crate::structs::AuthError;
use axum::extract::Json;
use axum::http::HeaderMap;
use tracing::{debug, span, Level};

use crate::structs::global_response;
use crate::structs::Claims;
use crate::structs::{
    LoginAO, LoginVO, PassChangeAO, PassChangeVO, RegisterAO, RegisterVO, UserInfo,
};
use headers::HeaderValue;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[tracing::instrument]
pub async fn register_handler(Json(payload): Json<Value>) -> Json<Value> {
    let req: RegisterAO = serde_json::from_value(payload).unwrap();
    debug!("receive params, req: {:?}", req);

    // should encrypt pass

    if req.email.is_empty()
        || req.nick_name.is_empty()
        || req.password.is_empty()
        || req.code.is_empty()
    {
        return global_response::new(global_response::ERROR_CODE_PARAM_INVALID, RegisterVO::new());
    }

    crate::service::auth::register_user(&req).await
}

#[tracing::instrument]
pub async fn login_handler(Json(payload): Json<Value>) -> Json<Value> {
    let req: LoginAO = serde_json::from_value(payload).unwrap();
    debug!("receive params, req: {:?}", req);

    if req.email.is_empty() || req.password.is_empty() {
        return global_response::new(global_response::ERROR_CODE_PARAM_INVALID, LoginVO::new());
    }

    crate::service::auth::login(&req).await
}

#[tracing::instrument]
pub async fn change_pass_handler(claims: Claims, Json(payload): Json<Value>) -> Json<Value> {
    let req: PassChangeAO = serde_json::from_value(payload).unwrap();
    debug!("receive params, req: {:?}", req);

    if req.email.is_empty() || req.new_pass.is_empty() || req.code.is_empty() {
        return global_response::new(
            global_response::ERROR_CODE_PARAM_INVALID,
            PassChangeVO::new(),
        );
    }

    if !claims.user_info.email.eq(&req.email) {
        return global_response::new(
            global_response::ERROR_CODE_PARAM_INVALID,
            PassChangeVO::new(),
        );
    }

    crate::service::auth::change_pass(&req).await
}

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

            let u = UserInfo::new();
            let claims = Claims::new(expire_time, u);
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
