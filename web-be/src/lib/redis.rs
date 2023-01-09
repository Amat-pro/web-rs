use crate::config::CONFIG;
use futures::executor::block_on;
use lazy_static::lazy_static;
use redis::aio::ConnectionManager;

lazy_static! {
    pub static ref REDIS_CONNECTION_MANAGER: ConnectionManager = {
        let redis_config = CONFIG.get_redis_config();
        let client = redis::Client::open(redis_config.get_standalone_url()).unwrap();

        let redis_connection_manager_r = block_on(ConnectionManager::new(client));

        match redis_connection_manager_r {
            Ok(redis_connection_manager) => redis_connection_manager,
            Err(e) => {
                panic!("lazy_static REDIS_CONNECTION_MANAGER err, {}", e);
            }
        }
    };
}
