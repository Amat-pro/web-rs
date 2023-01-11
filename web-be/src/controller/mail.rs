use axum::extract::Json;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tracing::warn;

pub async fn send_mail_code_handler(Json(payload): Json<Value>) -> Json<Value> {
    let req: SendMailCodeAO = serde_json::from_value(payload).unwrap();

    let to = req.get_to();
    // check param
    if to.is_empty() {
        return Json(json!({
            "code":200,
            "message":"success",
            "payload":"",
        }));
    }
    // check others
    // ...

    // send_mail
    let send_mail_r = crate::lib::send_mail(
        to.clone(),
        "WEB-RS 发送邮箱验证码".to_string(),
        "验证码为xxx".to_string(),
    );
    match send_mail_r {
        Ok(_) => Json(json!({
            "code":200,
            "message":"success",
            "payload":"",
        })),
        Err(e) => {
            warn!("send mail to {} fail, err: {}", to, e);
            Json(json!({
                "code":10000,
                "message":"error",
                "payload":"",
            }))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct SendMailCodeAO {
    to: String,
}

impl SendMailCodeAO {
    fn get_to(&self) -> String {
        self.to.clone()
    }
}
