use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigProperty {
    server: ServerConfig,
    log: LogConfig,
    security: SecurityConfig,
    mail: MailConfig,
    mysql: MysqlConfig,
    mongodb: MongoDbConfig,
    redis: RedisConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerConfig {
    name: String,
    port: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MailConfig {
    smtp_server: String,
    user_name: String,
    password: String,
}

/// param for time.Duration: secs
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MysqlConfig {
    url: String,
    max_connections: u32,
    min_connections: u32,
    max_lifetime: u32,
    idle_timeout: u32,
    acquire_timeout: u32,
    // fair: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MongoDbConfig {
    standalone_url: String,
    database: String,
    max_idle_time: u32,
    min_pool_size: u32,
    max_pool_size: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RedisConfig {
    standalone_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogConfig {
    max_level: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SecurityConfig {
    jwt: JwtConfig,
}

/// param for time.Duration: milliseconds
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JwtConfig {
    secret: String,
    exp: u64,
}

impl ConfigProperty {
    pub fn get_server_config(&self) -> ServerConfig {
        self.server.clone()
    }

    pub fn get_log_config(&self) -> LogConfig {
        self.log.clone()
    }

    pub fn get_security_config(&self) -> SecurityConfig {
        self.security.clone()
    }

    pub fn get_mysql_config(&self) -> MysqlConfig {
        self.mysql.clone()
    }

    pub fn get_mongo_config(&self) -> MongoDbConfig {
        self.mongodb.clone()
    }

    pub fn get_mail_config(&self) -> MailConfig {
        self.mail.clone()
    }

    pub fn get_redis_config(&self) -> RedisConfig {
        self.redis.clone()
    }
}

impl ServerConfig {
    pub fn get_port(&self) -> usize {
        self.port
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl MailConfig {
    pub fn get_smtp_server(&self) -> String {
        self.smtp_server.clone()
    }

    pub fn get_user_name(&self) -> String {
        self.user_name.clone()
    }

    pub fn get_password(&self) -> String {
        self.password.clone()
    }
}

impl RedisConfig {
    pub fn get_standalone_url(&self) -> String {
        self.standalone_url.clone()
    }
}

impl MongoDbConfig {
    pub fn get_standalone_url(&self) -> String {
        self.standalone_url.clone()
    }
    pub fn get_database(&self) -> String {
        self.database.clone()
    }
    pub fn get_max_idle_time(&self) -> u32 {
        self.max_idle_time
    }
    pub fn get_min_pool_size(&self) -> u32 {
        self.min_pool_size
    }
    pub fn get_max_pool_size(&self) -> u32 {
        self.max_pool_size
    }
}

impl MysqlConfig {
    pub fn get_url(&self) -> String {
        self.url.clone()
    }
    pub fn get_max_connections(&self) -> u32 {
        self.max_connections
    }
    pub fn get_min_connections(&self) -> u32 {
        self.min_connections
    }
    pub fn get_max_lifetime(&self) -> u32 {
        self.max_lifetime
    }
    pub fn get_idle_timeout(&self) -> u32 {
        self.idle_timeout
    }
    pub fn get_acquire_timeout(&self) -> u32 {
        self.acquire_timeout
    }
    // pub fn get_fair(&self) -> bool {
    //     self.fair
    // }
}

impl SecurityConfig {
    pub fn get_jwt_config(&self) -> JwtConfig {
        self.jwt.clone()
    }
}

impl JwtConfig {
    pub fn get_secret(&self) -> String {
        self.secret.clone()
    }
    pub fn get_exp(&self) -> u64 {
        self.exp
    }
}

impl LogConfig {
    pub fn get_max_level(&self) -> tracing::Level {
        match self.max_level.as_str() {
            "TRACE" => tracing::Level::TRACE,
            "DEBUG" => tracing::Level::DEBUG,
            "INFO" => tracing::Level::INFO,
            "WARN" => tracing::Level::WARN,
            "ERROR" => tracing::Level::ERROR,
            _ => tracing::Level::DEBUG,
        }
    }
}
