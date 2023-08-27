use self::config::{Profile, print_profile};
use std::process::Command;
use std::io::{self, Write};

pub mod config;

pub fn run_profile(profile: Profile) {
    println!("Running profile: {}", profile.profile_name);
    print!("Executing command: ");
    print_profile(&profile);
    if profile.additionals.len() > 0 {
        let child = Command::new("cmd")
            .args(["/C", format!("{}\\{}", profile.path, profile.file).as_str()])
            .args(["-a", profile.algorithm.as_str()])
            .args(["-o", profile.url.as_str()])
            .args(["-u", format!("{}:{}.{}", profile.coin, profile.wallet_address, profile.worker_name).as_str()])
            .args(["-d", profile.devices.as_str()])
            .args(["", profile.additionals.as_str()])
            .spawn()
            .expect("failed to execute process...");
        //wait_for_child(child);
    } else {
        let child = Command::new("cmd")
            .args(["/C", format!("{}\\{}", profile.path, profile.file).as_str()])
            .args(["-a", profile.algorithm.as_str()])
            .args(["-o", profile.url.as_str()])
            .args(["-u", format!("{}:{}.{}", profile.coin, profile.wallet_address, profile.worker_name).as_str()])
            .args(["-d", profile.devices.as_str()])
            .spawn()
            .expect("failed to execute process...");
        //wait_for_child(child);
    }
}

fn wait_for_child(child: std::process::Child) {
    match child.wait_with_output() {
        Ok(out) => {
            println!("Process exited with success.");
            io::stdout().write_all(&out.stdout).unwrap();
        },
        Err(err) => {
            println!("Process exited with errors: {}", &err.to_string());
        },
    };
}