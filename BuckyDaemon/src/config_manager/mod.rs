extern crate serde_json;
use self::serde_json::Value;
use std::fs::File;
use std::io::Read;
use std::error::Error;
use std::env;
pub struct ConfigManager {
    x: String,
}

pub struct DatabaseConfig {
    databaseType: String,
    databaseAddress: String,
    databasePort: String,
    databaseUser: String,
    databasePassword: String,
}
pub struct ServerConfig {
    serverName: String,
    serverAddress: String,
    maxConcurrentConnections: u8,
    maxConcurrentJobsRunning: u8,
}

pub struct Config {
    serverCfg: ServerConfig,
    databaseCfg: DatabaseConfig,
}

// A Simple Implementation To Get The Config Object
impl ConfigManager {
    pub fn load(config_to_load: String) -> ConfigManager {
        // FileLoad
        let mut file = match File::open(&config_to_load) {
            Err(err_desc) => {
                panic!("Failed To Load BuckyDaemon Main Config (Reason: {})",
                       Error::description(&err_desc))
            }
            Ok(file) => file,
        };
        let mut data = String::new();
        file.read_to_string(&mut data);
        let configuration = ConfigManager::build_config(serde_json::from_str(&data.to_string())
                                                            .unwrap());
        ConfigManager { x: "".to_string() }
    }

    fn build_config(data: Value) {
        let server_config = data.as_object().unwrap().get("serverConfig");

    }
}
