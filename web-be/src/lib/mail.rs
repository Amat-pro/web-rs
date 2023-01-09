use crate::config::CONFIG;
use lazy_static::lazy_static;
use lettre::transport::smtp::{authentication::Credentials, Error};
use lettre::{Message, SmtpTransport, Transport};

lazy_static! {
    // SmtpTransport: will creates a new connection directly usable to send emails each time
    pub static ref MAILER: SmtpTransport = {
        let mail_config = CONFIG.get_mail_config();
        let creds = Credentials::new(mail_config.get_user_name(), mail_config.get_password());

        // Open a remote connection to gmail
        let mailer = SmtpTransport::relay(mail_config.get_smtp_server().as_str())
            .unwrap()
            .credentials(creds)
            .build();

        mailer
    };
}

pub fn send_mail(to: String, subject: String, body: String) -> std::result::Result<(), Error> {
    let mail_config = CONFIG.get_mail_config();

    let from = format!("{} <{}>", "WEB-RS", mail_config.get_user_name());

    let email = Message::builder()
        .from(from.parse().unwrap())
        // reply-to: 信件回复的收件人. 用户直接回复邮件时, reply-to 就是默认的收件人. 如果用户不指定它, from 就是默认的收件人.
        // .reply_to("Reply-To <luzhongbo@aliyun.com>".parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .body(body)
        .unwrap();

    match self::MAILER.send(&email) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
