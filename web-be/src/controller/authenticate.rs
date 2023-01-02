use crate::jwt::{authorize, AuthBody, AuthError};
use axum::extract::Json;
use axum::http::HeaderMap;
use tracing::{debug, span, Level};

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
