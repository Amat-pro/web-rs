use crate::controller::{authenticate_handler, hello_world_handler, protected_hello_world_handler};
use axum::{routing::post, Router};

pub fn create_router() -> Router {
    Router::new()
        .route("/hello-world", post(hello_world_handler))
        .route("/authenticate", post(authenticate_handler))
        .route("/protected/hello-world", post(protected_hello_world_handler))
}
