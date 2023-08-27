use std::fs;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    pub profiles: Vec<Profile>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Profile{
    pub profile_name: String,
    pub path: String,
    pub file: String,
    pub algorithm: String,
    pub url: String,
    pub coin: String,
    pub wallet_address: String,
    pub worker_name: String,
    pub devices: String,
    pub additionals: String,
}

pub fn print_config(configs: Config) {
    configs.profiles.iter().for_each(|p| {
        print_profile(p);
    });
}

pub fn print_profile(p: &Profile) {
    if p.additionals.len() > 0 {
        println!("{}\\{} -a {} -o {} -u {}:{}.{} -d {} {}", p.path, p.file, p.algorithm, p.url, p.coin, p.wallet_address, p.worker_name, p.devices, p.additionals);
    }else {
        println!("{}\\{} -a {} -o {} -u {}:{}.{} -d {}", p.path, p.file, p.algorithm, p.url, p.coin, p.wallet_address, p.worker_name, p.devices);
    }
}

pub fn read_profiles_from_config(verbose: bool) -> Config {
    let config_json: String = fs::read_to_string("config.json").expect("The file: 'config.json' was not found.");    
    if verbose {
        println!("\n\nPrinting content read from config.json");
        println!("{}", config_json);
    }

    let deserialized_config: Config = serde_json::from_str(&config_json).unwrap();
    
    let mut configs = Config{ profiles : vec![] };
    configs.profiles = deserialized_config.profiles.clone();
    if verbose {
        println!("\n\nPrinting loaded config");
        println!("{:?}\n\n", configs);
    }
    configs.clone()
}