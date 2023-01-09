mod authenticate;
mod hello_world;
mod mail;

pub use authenticate::authenticate_handler;
pub use hello_world::{
    hello_world_handler, protected_hello_world_handler, test_mongodb_handler, test_mysql_handler,
    test_redis_cmd_handler,
};
pub use mail::send_mail_code_handler;
