pub mod config;
pub mod profile;
pub mod cmd_args;

use std::process::Command;
use std::fs;
use serde_json;

use crate::mining::cmd_args::build_cmd;
use crate::mining::config::Config;
use crate::mining::profile::{print_profile, Profile};

pub fn run_profile(profile: Profile) {
    println!("Running profile: {}", profile.profile_name);
    print!("Executing command: ");
    print_profile(&profile);
    let status = Command::new("cmd")
        .args([build_cmd(&profile)])
        .status()
        .expect("failed to execute process...");
    println!("CPU thread running miner [with profile: {}] finished with: {}", profile.profile_name, status);
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