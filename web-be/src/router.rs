use crate::controller::{
    authenticate_handler, hello_world_handler, protected_hello_world_handler,
    send_mail_code_handler, test_redis_cmd_handler,
};
use axum::{routing::post, Router};

pub fn create_router() -> Router {
    Router::new()
        .route("/hello-world", post(hello_world_handler))
        .route("/authenticate", post(authenticate_handler))
        .route(
            "/protected/hello-world",
            post(protected_hello_world_handler),
        )
        .route("/test-redis-cmd", post(test_redis_cmd_handler))
        .route("/mail-code/send", post(send_mail_code_handler))
}
