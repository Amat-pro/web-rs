use std::time::Duration;

use crate::config::CONFIG;
use futures::executor::block_on;
use lazy_static::lazy_static;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::env;

lazy_static! {
    pub static ref MYSQL_POOL: MySqlPool = {
        let mysql_config = CONFIG.get_mysql_config();

        let _: Result<String, ()> = env::var("DATABASE_URL").or_else(|_e| {
            env::set_var("DATABASE_URL", mysql_config.get_url());
            Ok(mysql_config.get_url())
        });

        let pool_r = block_on(
            MySqlPoolOptions::new()
                .max_connections(mysql_config.get_max_connections())
                .min_connections(mysql_config.get_min_connections())
                .max_lifetime(Some(Duration::from_secs(
                    mysql_config.get_max_lifetime() as u64
                )))
                .idle_timeout(Some(Duration::from_secs(
                    mysql_config.get_idle_timeout() as u64
                )))
                .acquire_timeout(Duration::from_secs(
                    mysql_config.get_acquire_timeout() as u64
                ))
                .connect(mysql_config.get_url().as_str()),
        );

        match pool_r {
            Ok(pool) => pool,
            Err(e) => {
                panic!("init mysql pool fail, err: {}", e);
            }
        }
    };
}
