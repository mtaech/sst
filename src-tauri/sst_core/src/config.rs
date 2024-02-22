use std::fs;
use log::{error, info};
use crate::model::{Config, Profile};
use crate::sub::{download_sub, get_sub};
use crate::utils::get_config_dir;

pub fn init_config() -> Config{
    let config_dir = get_config_dir();
    let config_path = config_dir.join("config.json");
    if !config_path.exists() {
        let config = Config::default();
        fs::write(&config_path,serde_json::to_string_pretty(&config).unwrap()).unwrap();
        return config;
    }
    let string = fs::read_to_string(config_path).unwrap();
    let config:Config = serde_json::from_str(&string).unwrap();
    return config;
}

pub fn get_config()->Config{
    let config_dir = get_config_dir();
    let config_path = config_dir.join("config.json");
    if !config_path.exists() {
        error!("not has config file");
        init_config();
    }
    println!("path {:?}",config_path);
    let string = fs::read_to_string(config_path).unwrap();
    let config:Config = serde_json::from_str(&string).unwrap();
    return config;
}

pub fn update_config(config:Config){
    let path = get_config_dir().join("config.json");
    fs::write(&path,serde_json::to_string_pretty(&config).unwrap()).unwrap();
}

///add profile
pub fn add_profile(url:String){
    let sub_str = download_sub(&url);
    let sub = get_sub(sub_str);
    let profile = Profile::from_sub(sub, url);
    let mut config = get_config();
    config.profiles.push(profile);
    update_config(config);
}