use super::config::ConfigProperty;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::env;
use std::fs::File;
use tracing::{debug, info, warn};

const DEFAULT_CONFIG_FILE_PATH: &'static str =
    "/Users/luzhongbo/VsCodeProjects/web-rs/web-be/src/config/config.yaml";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct YamlConfig;

impl YamlConfig {
    pub fn new() -> Self {
        info!("=========================================== YamlConfig ===========================================");
        YamlConfig {}
    }

    pub fn init_config(&self) -> Result<super::config::ConfigProperty, ()> {
        let path = env::var("config_file_path")
            .or_else(|_e| -> Result<String, ()> { Ok(DEFAULT_CONFIG_FILE_PATH.to_string()) })
            .unwrap();

        let f_r = File::open(path);
        match f_r {
            Ok(f) => {
                let config_r: serde_yaml::Result<ConfigProperty> = serde_yaml::from_reader(f);

                let result: Result<ConfigProperty, ()>;

                match config_r {
                    Ok(config) => {
                        debug!("init_config success, config: {:?}", config);
                        result = Ok(config)
                    }
                    Err(err) => {
                        warn!("init_config: parse config from yaml file err{}", err);
                        result = Err(());
                    }
                }
                return result;
            }
            Err(e) => {
                warn!("init_config: open file err{}", e);
                return Err(());
            }
        }
    }
}
