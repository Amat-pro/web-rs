use crate::{
    jwt::Claims,
    lib::{MONGODB_CLIENT, REDIS_CONNECTION_MANAGER},
};
use axum::extract::Json;
use redis::RedisError;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tracing::{info, span, warn, Level};

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

    let r2: Result<String, RedisError> = redis::cmd("GET")
        .arg("web-rs:test:1")
        .query_async(&mut REDIS_CONNECTION_MANAGER.clone())
        .await;

    match r2 {
        Ok(v) => {
            info!(
                "test_redis_cmd_handler, get web-rs:test:1 success, val is `{}`",
                v
            );
        }
        Err(e) => {
            warn!("test_redis_cmd_handler, get web-rs:test:1 err: {}", e);
        }
    }

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

pub async fn test_mongodb_handler() -> Json<Value> {
    let client = MONGODB_CLIENT.clone();
    let db = client.database("Mytest");

    let collection_names_r = db.list_collection_names(None).await;
    match collection_names_r {
        Ok(collection_names) => {
            info!(
                "test_mongodb_handler, collection_names: {:?}",
                collection_names
            );
        }
        Err(e) => {
            warn!(
                "test_mongodb_handler, list collection names fail, err: {}",
                e
            );
        }
    }

    // Get a handle to a collection of `Book`.
    let typed_collection = db.collection::<Book>("books");
    let books = vec![
        Book {
            title: "The Grapes of Wrath".to_string(),
            author: "John Steinbeck".to_string(),
        },
        Book {
            title: "To Kill a Mockingbird".to_string(),
            author: "Harper Lee".to_string(),
        },
    ];

    // Insert the books into "mydb.books" collection, no manual conversion to BSON necessary.
    let insert_many_r = typed_collection.insert_many(books, None).await;
    match insert_many_r {
        Ok(_) => Json(json!({
            "code":200,
            "message":"success",
            "payload":"",
        })),
        Err(e) => {
            warn!(
                "test_mongodb_handler, insert many to Mytest.books fail, err: {}",
                e
            );
            Json(json!({
                "code":10000,
                "message":"insert many to Mytest.books fail",
                "payload":"",
            }))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    title: String,
    author: String,
}
