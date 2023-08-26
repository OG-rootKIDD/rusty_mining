pub mod config;

use std::fs;
use config::{ Config, Profile };
use serde_json;

pub fn print_config(configs: Config) {
    configs.profiles.iter().for_each(|p| {
        print_profile(p);
    });
}

fn print_profile(p: &Profile) {
    println!("Printing miner profile: {}", p.profile_name);
    println!("{}\\{} -a {} -o {} -u {}:{}.{} -d {} {}", p.path, p.file, p.algorithm, p.url, p.coin, p.wallet_address, p.worker_name, p.devices, p.additionals);
}

pub fn read_profiles_from_config(configs : &mut Config, verbose: bool) {
    let config_json: String = fs::read_to_string("config.json").expect("The file: 'config.json' was not found.");    
    if verbose {
        println!("\n\nPrinting content read from config.json");
        println!("{}", config_json);
    }

    let deserialized_config: Config = serde_json::from_str(&config_json).unwrap();
    configs.profiles = deserialized_config.profiles.clone();
    if verbose {
        println!("\n\nPrinting loaded config");
        println!("{:?}\n\n", configs);
    }
}