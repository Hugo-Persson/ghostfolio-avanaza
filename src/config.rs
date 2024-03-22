use std::collections::HashMap;

use crate::ghostfolio::GhostfolioConfig;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub ghostfolio: Option<GhostfolioConfig>,
    pub avanza_to_ghostfolio_ticker: HashMap<String, String>,
}

pub const DEFAULT_CONFIG_PATH: &str = "~/.avanza-ghostfolio-cli/config.json";
const CONFIG_DIR_NAME: &str = ".avanza-ghostfolio-cli";
impl Config {
    pub fn new() -> Config {
        let home_dir = dirs::home_dir().expect("Failed to get home dir");
        if !home_dir.join(CONFIG_DIR_NAME).exists() {
            std::fs::create_dir_all(home_dir.join(CONFIG_DIR_NAME))
                .expect("Failed to create config dir");
        }
        let config_path = home_dir.join(format!("{}/config.json", CONFIG_DIR_NAME));
        println!("Config path: {:?}", config_path);
        let config = if config_path.exists() {
            let config_file = std::fs::File::open(config_path).expect("Failed to open config file");

            serde_json::from_reader(config_file).expect("Failed to parse config file")
        } else {
            let config = Config {
                ghostfolio: None,
                avanza_to_ghostfolio_ticker: HashMap::new(),
            };
            let config_file =
                std::fs::File::create(config_path).expect("Failed to create config file");

            serde_json::to_writer(config_file, &config).expect("Failed to write config file");
            config
        };
        config
    }
    pub fn save(&self) {
        let config_path = dirs::home_dir()
            .expect("Failed to get home dir")
            .join(".avanza-ghostfolio-cli/config.json");
        let config_file = std::fs::File::create(config_path).expect("Failed to create config file");
        serde_json::to_writer(config_file, &self).expect("Failed to write config file");
    }
}
