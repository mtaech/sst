use std::fs;
use std::path::PathBuf;
use home::home_dir;

#[cfg(target_os = "linux")]
pub fn get_app_dir()->PathBuf{
    let home = home_dir().unwrap();
    let app_dir = home.join(".local").join("share")
        .join("sst");
    if !app_dir.exists() {
        fs::create_dir_all(&app_dir).unwrap();
    };
    return app_dir;
}

#[cfg(target_os = "linux")]
pub fn get_config_dir()->PathBuf{
    let home = home_dir().unwrap();
    let app_dir = home.join(".config").join("sst");
    if !app_dir.exists() {
        fs::create_dir_all(&app_dir).unwrap();
    };
    return app_dir;
}