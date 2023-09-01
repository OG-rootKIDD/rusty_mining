pub mod config;
pub mod profile;
pub mod cmd_args;

use crate::mining::config::Config;

use std::fs;
use std::time::SystemTime;
use serde_json;

pub fn read_profiles_from_config() -> Config {
    let config_json: String = fs::read_to_string("config.json").expect("The file: 'config.json' was not found.");    
    let deserialized_config: Config = serde_json::from_str(&config_json).expect("Could not deserialize config.json");
    let mut configs = Config{ profiles : vec![] };
    configs.profiles = deserialized_config.profiles.clone();
    configs.clone()
}

pub fn config_last_modified() -> SystemTime{
    fs::metadata("config.json")
        .expect("Could not get last modified for config")
        .modified()
        .expect("Could not read last modified for config.")
}