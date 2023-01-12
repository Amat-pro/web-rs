use axum::extract::Json;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::warn;

// todo router
pub async fn send_mail_code_handler(Json(payload): Json<Value>) -> Json<Value> {
    let req: SendMailCodeAO = serde_json::from_value(payload).unwrap();

    let to = req.get_to();
    // check param
    if to.is_empty() {
        return crate::structs::global_response::new(
            crate::structs::global_response::ERROR_CODE_ERROR,
            SendMailCodeVO::new(),
        );
    }
    // check others
    // ...

    // generate code
    let code = "1234".to_string();

    let set_email_code_r =
        crate::repository::redis::set_email_code_with_default_expire(&code, &to).await;
    match set_email_code_r {
        Ok(_) => {
            // send_mail
            let send_mail_r = crate::lib::send_mail(
                to.clone(),
                "WEB-RS 发送邮箱验证码".to_string(),
                "验证码为xxx".to_string(),
            );
            match send_mail_r {
                Ok(_) => crate::structs::global_response::new(
                    crate::structs::global_response::ERROR_CODE_SUCCESS,
                    SendMailCodeVO::new(),
                ),
                Err(e) => {
                    warn!("send mail to {} fail, err: {}", to, e);
                    crate::structs::global_response::new(
                        crate::structs::global_response::ERROR_CODE_ERROR,
                        SendMailCodeVO::new(),
                    )
                }
            }
        }
        Err(e) => {
            warn!("send_mail_code_handler, set mail code err: {}", e);
            crate::structs::global_response::new(
                crate::structs::global_response::ERROR_CODE_ERROR,
                SendMailCodeVO::new(),
            )
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct SendMailCodeAO {
    to: String,
}

impl SendMailCodeAO {
    fn get_to(&self) -> String {
        self.to.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct SendMailCodeVO {}

impl SendMailCodeVO {
    fn new() -> Self {
        Self {}
    }
}
