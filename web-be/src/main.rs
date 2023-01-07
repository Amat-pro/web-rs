mod ao;
mod controller;
mod router;
mod service;
mod vo;
use std::net::SocketAddr;
use tokio::signal;
mod config;
mod init;
mod jwt;
mod lib;

#[tokio::main]
async fn main() {
    // init
    init::init();

    // build our application with a route
    let router = router::create_router();

    // run it
    let server_config = config::CONFIG.clone().get_server_config();
    let addr = SocketAddr::from(([0, 0, 0, 0], server_config.get_port() as u16));
    tracing::info!(
        "Application `{}` listening on `{}`",
        server_config.get_name(),
        addr
    );
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown(Do Others Here!).");
}
