use crate::config::CONFIG;
use futures::executor::block_on;
use lazy_static::lazy_static;
use mongodb::{options::ClientOptions, Client};

lazy_static! {
    pub static ref MONGODB_CLIENT: Client = {
        let mongodb_config = CONFIG.get_mongo_config();
        let client_options_r = block_on(ClientOptions::parse(mongodb_config.get_standalone_url()));

        match client_options_r {
            Ok(mut client_options) => {
                // can set other options here like ```client_options.app_name = Some("My App".to_string());```
                client_options.app_name = Some("WEB-RS".to_string());
                let client_r = Client::with_options(client_options);
                match client_r {
                    Ok(client) => {
                        client
                    },
                    Err(e) => {
                        panic!("init mongodb client fail, err: {}", e);
                    }
                }
            },
            Err(e) => {
                panic!("init mongodb client options fail, err: {}", e);
            }
        }
    };

}
