mod mail;
mod mongodb;
mod mysql;
mod redis;

pub use mail::send_mail;
pub use mail::MAILER;

pub use crate::lib::mongodb::MONGODB_CLIENT;
pub use crate::lib::redis::REDIS_CONNECTION_MANAGER;