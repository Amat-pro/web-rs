mod article;
mod auth;
mod hello_world;
mod mail;

pub use auth::{authenticate_handler, change_pass_handler, login_handler, register_handler};
pub use hello_world::{
    hello_world_handler, protected_hello_world_handler, test_mongodb_handler, test_mysql_handler,
    test_redis_cmd_handler,
};
pub use mail::send_mail_code_handler;

pub use article::{create_article_handler, list_mine_article_handler, search_article_handler};
