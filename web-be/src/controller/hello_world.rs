use crate::{jwt::Claims, lib::REDIS_CONNECTION_MANAGER};
use axum::extract::Json;
use serde_json::{json, Value};
use tracing::{info, span, Level};

// Content-Type: application/json
pub async fn hello_world_handler(Json(payload): Json<Value>) -> Json<Value> {
    // request
    let span = span!(Level::INFO, "hello_world_handler", "trace_id: {}", 10000);
    let _enter = span.enter();

    let req: crate::ao::HelloWorldAO = serde_json::from_value(payload).unwrap();
    info!("receive params, req: {:?}", req);

    crate::service::do_something();

    // response
    let res = crate::vo::HelloWorldVO::new("desc".to_string(), 18);
    Json(json!({
        "code":200,
        "message":"success",
        "payload":res,
    }))
}

// Content-Type: application/json
pub async fn protected_hello_world_handler(
    claims: Claims,
    Json(payload): Json<Value>,
) -> Json<Value> {
    let span = span!(
        Level::INFO,
        "protected_hello_world_handler",
        "trace_id: {}",
        10001
    );
    let _enter = span.enter();

    let req: crate::ao::HelloWorldAO = serde_json::from_value(payload).unwrap();
    info!("receive params, req: {:?}, claims: {:?}", req, claims);
    info!("receive params, claims: {:?}", claims);

    // response
    let res = crate::vo::HelloWorldVO::new("desc".to_string(), 19);
    Json(json!({
        "code":200,
        "message":"success",
        "payload":res,
    }))
}

pub async fn test_redis_cmd_handler() -> Json<Value> {
    let r = redis::cmd("SET")
        .arg("web-rs:test:1")
        .arg("value")
        .query_async(&mut REDIS_CONNECTION_MANAGER.clone())
        .await;

    match r {
        Ok(()) => Json(json!({
            "code":200,
            "message":"success",
            "payload":"",
        })),
        Err(e) => Json(json!({
            "code":1000,
            "message":"redis command err",
            "payload":"",
        })),
    }
}
