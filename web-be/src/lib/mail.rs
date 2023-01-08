use crate::config::CONFIG;
use lazy_static::lazy_static;
use lettre::transport::smtp::{authentication::Credentials, Error};
use lettre::{Message, SmtpTransport, Transport};

lazy_static! {
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

    let email = Message::builder()
        .from(mail_config.get_user_name().parse().unwrap())
        // .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .body(body)
        .unwrap();

    match self::MAILER.send(&email) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
