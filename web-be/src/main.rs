mod config;
mod controller;
mod init;
mod lib;
mod repository;
mod router;
mod service;
mod structs;
mod utils;
use std::net::SocketAddr;
use tokio::signal;

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
        .with_graceful_shutdown(graceful_shutdown())
        .await
        .unwrap();
}

async fn graceful_shutdown() {
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
    let _ = crate::lib::MYSQL_POOL.close();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        // 4294967295
        println!("{}", std::u32::MAX);
        // 18446744073709551615
        println!("{}", std::u64::MAX);
    }
}
