use rusty_mining::mining::{ print_config, read_profiles_from_config, config::Config };

fn main() {
    println!("Rusty Mining");
    let mut miner_config = Config{ profiles : vec![] };
    read_profiles_from_config(&mut miner_config, false);
    print_config(miner_config);
}