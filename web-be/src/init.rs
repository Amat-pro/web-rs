use tracing::info;

use crate::config::CONFIG;

pub fn init() {
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();

    // init config
    info!("do init config, {:?}", CONFIG.clone());

    // do other inits
}
