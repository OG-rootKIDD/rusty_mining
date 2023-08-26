use serde::{Serialize, Deserialize};

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