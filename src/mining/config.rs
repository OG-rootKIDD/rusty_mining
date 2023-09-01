use crate::mining::{ profile::print_profile, profile::Profile};
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    pub profiles: Vec<Profile>
}

pub fn print_config(configs: Config) {
    configs.profiles.iter().for_each(|p| {
        print_profile(p.clone());
    });
}