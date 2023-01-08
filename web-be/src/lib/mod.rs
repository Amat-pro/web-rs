mod mail;
mod mysql;
mod mongodb;
mod redis;

pub use mail::MAILER;
pub use mail::send_mail;
