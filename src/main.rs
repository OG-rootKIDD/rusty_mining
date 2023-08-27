use rusty_mining::mining::{ run_profile, config::read_profiles_from_config };
use std::{thread, time::Duration};

fn main() {
    println!("Rusty Mining");
    let miner_config = read_profiles_from_config(false);
    miner_config.profiles.iter().for_each(|profile| {
        let local_thread_profile = profile.clone();
        let handle = thread::spawn(|| {
            run_profile(local_thread_profile);
        });
        while handle.is_finished() != true {
            thread::sleep(Duration::from_secs(10));
        }
    });
}