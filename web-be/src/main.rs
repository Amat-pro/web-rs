mod ao;
mod vo;

use axum::extract::Json;
use axum::{routing::post, Router};
use serde_json::{json, Value};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/hello-world", post(hello_world_handler));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Content-Type: application/json
async fn hello_world_handler(Json(payload): Json<Value>) -> Json<Value> {
    // request
    println!("=============>> {:?}", payload);

    let req: ao::HelloWorldAO = serde_json::from_value(payload).unwrap();
    println!("=============>> req: {:?}", req);

    // response
    let res = vo::HelloWorldVO::new("desc".to_string(), 18);
    Json(json!({
        "code":200,
        "message":"success",
        "payload":res,
    }))
}
