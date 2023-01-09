use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigProperty {
    server: ServerConfig,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MysqlConfig {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MongoDbConfig {
    standalone_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RedisConfig {
    standalone_url: String,
}

impl ConfigProperty {
    pub fn get_server_config(&self) -> ServerConfig {
        self.server.clone()
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
}
