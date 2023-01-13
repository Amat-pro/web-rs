use crate::config::CONFIG;
use futures::executor::block_on;
use lazy_static::lazy_static;
use redis::aio::ConnectionManager;
use redis::{RedisResult, Value};

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

pub async fn set(key: &String, value: &String) -> RedisResult<Value> {
    redis::cmd("SET")
        .arg(key)
        .arg(value)
        .query_async(&mut REDIS_CONNECTION_MANAGER.clone())
        .await
}

pub async fn pexpire(key: &String, expire: u64) -> RedisResult<Value> {
    redis::cmd("PEXPIRE")
        .arg(key)
        .arg(expire)
        .query_async(&mut REDIS_CONNECTION_MANAGER.clone())
        .await
}

pub async fn set_with_secs_expire(
    key: &String,
    value: &String,
    secs: usize,
) -> RedisResult<(Value)> {
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
) -> RedisResult<Value> {
    redis::cmd("SET")
        .arg(key)
        .arg(value)
        .arg("PX")
        .arg(millis)
        .query_async(&mut REDIS_CONNECTION_MANAGER.clone())
        .await
}

pub async fn set_nx_with_secs_expire(
    key: &String,
    value: &String,
    secs: usize,
) -> RedisResult<Value> {
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
) -> RedisResult<Value> {
    redis::cmd("SET")
        .arg(key)
        .arg(value)
        .arg("PX")
        .arg(millis)
        .arg("NX")
        .query_async(&mut REDIS_CONNECTION_MANAGER.clone())
        .await
}

pub async fn set_xx_with_secs_expire(
    key: &String,
    value: &String,
    secs: usize,
) -> RedisResult<Value> {
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
) -> RedisResult<Value> {
    redis::cmd("SET")
        .arg(key)
        .arg(value)
        .arg("PX")
        .arg(millis)
        .arg("XX")
        .query_async(&mut REDIS_CONNECTION_MANAGER.clone())
        .await
}

pub async fn get(key: &String) -> RedisResult<Value> {
    redis::cmd("GET")
        .arg(key)
        .query_async(&mut REDIS_CONNECTION_MANAGER.clone())
        .await
}

#[cfg(test)]
mod tests {

    use super::{get, set};
    use tokio::runtime;

    #[test]
    fn test_set() {
        let rt = runtime::Runtime::new().unwrap();

        let set_r = rt.block_on(set(&"test-set".to_string(), &"".to_string()));
        match set_r {
            Ok(v) => match v {
                redis::Value::Int(i) => println!("{}", i),
                redis::Value::Data(_) => println!("Data"),
                redis::Value::Bulk(_) => println!("Bulk"),
                redis::Value::Status(_) => println!("Status"),
                redis::Value::Nil => println!("Nil"),
                redis::Value::Okay => println!("Ok"),
            },
            Err(e) => {
                println!("err: {}", e);
            }
        }
    }

    #[test]
    fn test_get() {
        let rt = runtime::Runtime::new().unwrap();

        let get_r = rt.block_on(get(&"fenrugrhe".to_string()));
        match get_r {
            Ok(v) => match v {
                redis::Value::Int(i) => println!("{}", i),
                redis::Value::Data(_) => println!("Data"),
                redis::Value::Bulk(_) => println!("Bulk"),
                redis::Value::Status(_) => println!("Status"),
                redis::Value::Nil => println!("Nil"),
                redis::Value::Okay => println!("Ok"),
            },
            Err(e) => {
                println!("err: {}", e);
            }
        }
    }
}
