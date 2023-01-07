mod config;
mod yaml_config;

use self::config::ConfigProperty;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CONFIG: ConfigProperty = {
        let config_r = yaml_config::YamlConfig::new().init_config();
        match config_r {
            Ok(config) => config,
            Err(e) => {
                panic!("init config from yaml file fail, {:?}", e);
            }
        }
    };
}
