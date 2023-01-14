use crate::structs::{SendMailCodeAO, SendMailCodeVO};
use axum::extract::Json;
use axum::http::HeaderMap;
use serde_json::Value;
use tracing::{info_span, Instrument};

// #[tracing::instrument]
pub async fn send_mail_code_handler(headers: HeaderMap, Json(payload): Json<Value>) -> Json<Value> {
    // request
    let request_id = super::get_trace_id_from_header(&headers);

    let span = info_span!("send_mail_code_handler enter, trace_id: {:?}", request_id);

    let req: SendMailCodeAO = serde_json::from_value(payload).unwrap();

    let to = req.get_to();
    let mail_type = req.get_mail_type();
    // check param
    if to.is_empty() {
        return crate::structs::global_response::new(
            crate::structs::global_response::ERROR_CODE_PARAM_INVALID,
            SendMailCodeVO::new(),
        );
    }

    crate::service::auth::send_email_with_default_limit(mail_type, &to)
        .instrument(span.clone())
        .await

    // crate::service::auth::send_email_with_default_limit(&to).await
}
