use rusty_mining::mining::{ run_profile, read_profiles_from_config, cmd_args::* };
use std::{thread, time::Duration, thread::JoinHandle };

fn main() {
    println!("Rusty Mining");
    let miner_config = read_profiles_from_config(false);
    miner_config.profiles.iter().for_each(|profile| {
        let local_thread_profile = profile.clone();
        run_profile(local_thread_profile);
    });
    //run_miners(miner_config);
}

fn run_miners(miner_config: rusty_mining::mining::config::Config) {
    let mut join_handles: Vec<JoinHandle<()>>= vec![];
    let mut active_handles = 0;
    miner_config.profiles.iter().for_each(|profile| {
        let local_thread_profile = profile.clone();
        let handle = thread::spawn(|| {
            run_profile(local_thread_profile);
        });
        join_handles.push(handle);
        active_handles += 1;
    });
    let mut active_handles_changed = false;
    while active_handles > 0 {
        for h in join_handles.iter() {
            if h.is_finished() {
                active_handles -= 1;
                active_handles_changed = true;
            }
        }
        if active_handles_changed {
            println!("Active CPU threads running miners: {}", active_handles);
            active_handles_changed = false;
        }
        thread::sleep(Duration::from_secs(1));
    }
}