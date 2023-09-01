use rusty_mining::mining::{ read_profiles_from_config, config_last_modified, cmd_args::build_cmd};
use std::{thread, time::{Duration, SystemTime}, process::Command};
use sysinfo::{ProcessExt, Signal, System, SystemExt};

fn main() {
    let mut config_read = config_last_modified();
    let mut start_miners = true;

    loop {
        let profiles = &read_profiles_from_config().profiles;
        if start_miners {
            profiles.iter().for_each(|profile| {
                let local_profile = profile.clone();
                println!("[Main Thread] Starting miner thread for profile: {}.", local_profile.profile_name);
                println!("[Mining Thread: {}] Initializing...", local_profile.profile_name);
                let _ = Command::new("cmd")
                    .args([build_cmd(&local_profile)])
                    .spawn()
                    .expect(format!("[Mining Thread: {}] Failed to start...", local_profile.profile_name).as_str());
            });
            start_miners = false;
        }

        if config_has_changed(config_read) {
            config_read = config_last_modified();
            println!("[Main Thread] Config has changed... Stopping miners...");
            let mut found = true;
            while found {
                found = false;
                let s = System::new_all();
                s.processes().iter().for_each(|p|{
                    profiles.iter().for_each(|profile| {
                        let local_profile = profile.clone();
                        let process_name = p.1.name();
                        if local_profile.file == process_name {
                            found = true;
                            println!("[Main Thread] killing process {}", process_name);
                            p.1.kill_with(Signal::Kill);
                        }
                    });
                });
                thread::sleep(Duration::from_secs(5));
            }
            start_miners = true;
            println!("[Main Thread] Idle for 1 minute before restarting miners.");
        }
        thread::sleep(Duration::from_secs(60));
    }
}

fn config_has_changed(config_read: SystemTime) -> bool {
    config_last_modified()
    .duration_since(config_read)
    .expect("[Main Thread] Could not compare config last modified times.")
    .is_zero() == false
}