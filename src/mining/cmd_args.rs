use crate::mining::profile::Profile;

pub enum CmdArgs {
    Executeable,
    Algorithm,
    Url,
    Coin,
    Devices,
    Additionals
}

pub fn build_cmd(profile: &Profile) -> String {
    let mut cmd = "".to_string();
    cmd.push_str(get_cmd_arg_option(CmdArgs::Executeable, profile).as_str());
    cmd.push_str(get_cmd_arg_option(CmdArgs::Algorithm, profile).as_str());
    cmd.push_str(get_cmd_arg_option(CmdArgs::Url, profile).as_str());
    cmd.push_str(get_cmd_arg_option(CmdArgs::Coin, profile).as_str());
    cmd.push_str(get_cmd_arg_option(CmdArgs::Devices, profile).as_str());
    cmd.push_str(get_cmd_arg_option(CmdArgs::Additionals, profile).as_str());
    cmd
}

pub fn get_cmd_arg_option(cmd_args: CmdArgs, profile: &Profile) -> String{
    match cmd_args {
        CmdArgs::Executeable => {
            let mut exe_path = "/C ".to_string();
            exe_path.push_str(profile.path.as_str());
            exe_path.push_str("\\");
            exe_path.push_str(profile.file.as_str());
            exe_path
        },
        CmdArgs::Algorithm => {
            let mut algo = " -a ".to_string();
            algo.push_str(profile.algorithm.as_str());
            algo
        },
        CmdArgs::Url => {
            let mut algo = " -o ".to_string();
            algo.push_str(profile.url.as_str());
            algo
        },
        CmdArgs::Coin => {
            let mut coin = " -u ".to_string();
            coin.push_str(profile.coin.as_str());
            coin.push_str(":");
            coin.push_str(profile.wallet_address.as_str());
            coin.push_str(".");
            coin.push_str(profile.worker_name.as_str());
            coin
        }
        CmdArgs::Devices => {
            let mut devices = " -d ".to_string();
            devices.push_str(profile.devices.as_str());
            devices
        },
        CmdArgs::Additionals => {
            if profile.additionals.len() > 0 {
                let mut add = " ".to_string();
                add.push_str(profile.additionals.as_str());
                add
            }else {
                "".to_string()
            }
        }
    }
}