use crate::structs::{global_response, SendMailCodeVO};
use axum::extract::Json;
use serde_json::Value;
use tracing::{info, warn};

// #[tracing:instrument]
pub async fn send_email_with_default_limit(to: &String) -> Json<Value> {
    let key = format!(
        "{}:{}:email-to:{}",
        crate::lib::redis::REDIS_PREFIX,
        crate::lib::redis::AUTH_PREFIX,
        to
    );

    let set_r = crate::lib::redis::set_nx_with_secs_expire(&key, &"1".to_string(), 120).await;

    if set_r.is_err() {
        warn!(
            "send_email_with_default_limit, check default limit use redis err: {:?}",
            set_r.err().unwrap()
        );
        return global_response::new(
            global_response::ERROR_CODE_EMAIL_SEND_ERROR,
            SendMailCodeVO::new(),
        );
    }

    match set_r.unwrap() {
        redis::Value::Okay => {
            let code = crate::utils::rand::generate_numbers(6);

            let key = generate_email_code_key(to);

            let set_r = crate::lib::redis::set_with_secs_expire(&key, &code, 5 * 60).await;
            if set_r.is_err() {
                warn!(
                    "send_email_with_default_limit, save expire code to redis err: {:?}",
                    set_r.err().unwrap()
                );
                return global_response::new(
                    global_response::ERROR_CODE_EMAIL_SEND_ERROR,
                    SendMailCodeVO::new(),
                );
            }

            let body = format!("邮箱验证码: {}", code);
            let send_r =
                crate::lib::send_mail(to.to_string(), "WEB-RS 发送邮箱验证码".to_string(), body);
            if send_r.is_err() {
                warn!(
                    "send_email_with_default_limit, send mail err: {:?}",
                    send_r.err().unwrap()
                );
                return global_response::new(
                    global_response::ERROR_CODE_EMAIL_SEND_ERROR,
                    SendMailCodeVO::new(),
                );
            }

            global_response::new(global_response::ERROR_CODE_SUCCESS, SendMailCodeVO::new())
        }
        _ => {
            info!("send_email_with_default_limit, check default limit use redis invalid");
            global_response::new(
                global_response::ERROR_CODE_EMAIL_SEND_INVALID,
                SendMailCodeVO::new(),
            )
        }
    }
}

fn generate_email_code_key(to: &String) -> String {
    format!(
        "{}:{}:email-to:{}:code",
        crate::lib::redis::REDIS_PREFIX,
        crate::lib::redis::AUTH_PREFIX,
        to
    )
}
