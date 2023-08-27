use serde::{Serialize, Deserialize};

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

pub fn print_profile(p: &Profile) {
    if p.additionals.len() > 0 {
        println!("{}\\{} -a {} -o {} -u {}:{}.{} -d {} {}", p.path, p.file, p.algorithm, p.url, p.coin, p.wallet_address, p.worker_name, p.devices, p.additionals);
    }else {
        println!("{}\\{} -a {} -o {} -u {}:{}.{} -d {}", p.path, p.file, p.algorithm, p.url, p.coin, p.wallet_address, p.worker_name, p.devices);
    }
}