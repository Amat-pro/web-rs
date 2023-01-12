use crate::config::CONFIG;
use futures::executor::block_on;
use lazy_static::lazy_static;
use redis::aio::ConnectionManager;
use redis::RedisResult;

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

pub async fn set(key: &String, value: &String) -> RedisResult<()> {
    redis::cmd("SET")
        .arg(key)
        .arg(value)
        .query_async(&mut REDIS_CONNECTION_MANAGER.clone())
        .await
}

pub async fn pexpire(key: &String, expire: u64) -> RedisResult<()> {
    redis::cmd("PEXPIRE")
        .arg(key)
        .arg(expire)
        .query_async(&mut REDIS_CONNECTION_MANAGER.clone())
        .await
}

pub async fn set_with_secs_expire(key: &String, value: &String, secs: usize) -> RedisResult<()> {
    redis::cmd("SET")
        .arg(key)
        .arg(value)
        .arg("EX")
        .arg(secs)
        .query_async(&mut REDIS_CONNECTION_MANAGER.clone())
        .await
}

pub async fn set_with_millis_expire(
    key: &String,
    value: &String,
    millis: usize,
) -> RedisResult<()> {
    redis::cmd("SET")
        .arg(key)
        .arg(value)
        .arg("PX")
        .arg(millis)
        .query_async(&mut REDIS_CONNECTION_MANAGER.clone())
        .await
}

pub async fn set_nx_with_secs_expire(key: &String, value: &String, secs: usize) -> RedisResult<()> {
    redis::cmd("SET")
        .arg(key)
        .arg(value)
        .arg("EX")
        .arg(secs)
        .arg("NX")
        .query_async(&mut REDIS_CONNECTION_MANAGER.clone())
        .await
}

pub async fn set_nx_with_millis_expire(
    key: &String,
    value: &String,
    millis: usize,
) -> RedisResult<()> {
    redis::cmd("SET")
        .arg(key)
        .arg(value)
        .arg("PX")
        .arg(millis)
        .arg("NX")
        .query_async(&mut REDIS_CONNECTION_MANAGER.clone())
        .await
}

pub async fn set_xx_with_secs_expire(key: &String, value: &String, secs: usize) -> RedisResult<()> {
    redis::cmd("SET")
        .arg(key)
        .arg(value)
        .arg("EX")
        .arg(secs)
        .arg("XX")
        .query_async(&mut REDIS_CONNECTION_MANAGER.clone())
        .await
}

pub async fn set_xx_with_millis_expire(
    key: &String,
    value: &String,
    millis: usize,
) -> RedisResult<()> {
    redis::cmd("SET")
        .arg(key)
        .arg(value)
        .arg("PX")
        .arg(millis)
        .arg("XX")
        .query_async(&mut REDIS_CONNECTION_MANAGER.clone())
        .await
}
