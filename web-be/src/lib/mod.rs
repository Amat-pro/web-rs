mod mail;
mod mongodb;
mod mysql;
pub mod redis;

pub use mail::send_mail;
pub use mail::MAILER;

pub use crate::lib::mongodb::MONGODB_CLIENT;
pub use crate::lib::mysql::MYSQL_POOL;
pub use crate::lib::redis::REDIS_CONNECTION_MANAGER;
