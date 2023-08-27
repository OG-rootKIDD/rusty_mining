use self::config::{Profile, print_profile};
use std::process::Command;

pub mod config;

pub fn run_profile(profile: Profile) {
    println!("Running profile: {}", profile.profile_name);
    print!("Executing command: ");
    print_profile(&profile);
    if profile.additionals.len() > 0 {
        let status = Command::new("cmd")
            .args(["/C", format!("{}\\{}", profile.path, profile.file).as_str()])
            .args(["-a", profile.algorithm.as_str()])
            .args(["-o", profile.url.as_str()])
            .args(["-u", format!("{}:{}.{}", profile.coin, profile.wallet_address, profile.worker_name).as_str()])
            .args(["-d", profile.devices.as_str()])
            .args(["", profile.additionals.as_str()])
            .status()
            .expect("failed to execute process...");
        println!("CPU thread finished with: {status}");
    } else {
        let status = Command::new("cmd")
            .args(["/C", format!("{}\\{}", profile.path, profile.file).as_str()])
            .args(["-a", profile.algorithm.as_str()])
            .args(["-o", profile.url.as_str()])
            .args(["-u", format!("{}:{}.{}", profile.coin, profile.wallet_address, profile.worker_name).as_str()])
            .args(["-d", profile.devices.as_str()])
            .status()
            .expect("failed to execute process...");
        println!("CPU thread running miner [with profile: {}] finished with: {}", profile.profile_name, status);
    }
}