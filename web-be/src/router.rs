use crate::controller::{
    authenticate_handler, change_pass_handler, create_article_handler, hello_world_handler,
    list_mine_article_handler, login_handler, protected_hello_world_handler, register_handler,
    search_article_handler, send_mail_code_handler, test_mongodb_handler, test_mysql_handler,
    test_redis_cmd_handler, update_article_handler,
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
        .route("/test-mongodb", post(test_mongodb_handler))
        .route("/test-mysql", post(test_mysql_handler))
        // =============================================== WEB-RS ===================================================
        .route("/mail-code/send", post(send_mail_code_handler))
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .route("/pass-change", post(change_pass_handler))
        .route("/article/create", post(create_article_handler))
        .route("/article/update", post(update_article_handler))
        .route("/article/search", post(search_article_handler))
        .route("/article/mine", post(list_mine_article_handler))
}
