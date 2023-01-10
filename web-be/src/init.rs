use tracing::info;

use crate::config::CONFIG;

pub fn init() {
    // init config
    let config = CONFIG.clone();

    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt()
        .with_max_level(config.get_log_config().get_max_level())
        .with_timer(tracing_subscriber::fmt::time::LocalTime::rfc_3339())
        .init();

    // init redis connection manager (like MultiplexedConnection)
    info!("do init redis connection manager");
    let _ = crate::lib::REDIS_CONNECTION_MANAGER.clone();

    // init mongodb client
    let _ = crate::lib::MONGODB_CLIENT;

    // do other inits
}
