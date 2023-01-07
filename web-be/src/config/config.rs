use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigProperty {
    server: ServerConfig,
    mail: MailConfig,
    mysql: MysqlConfig,
    mongodb: MongoDbConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerConfig {
    name: String,
    port: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MailConfig {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MysqlConfig {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MongoDbConfig {}

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
}

impl ServerConfig {
    pub fn get_port(&self) -> usize {
        self.port
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}
