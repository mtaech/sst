use std::fs;
use rust_embed::RustEmbed;
use crate::utils::get_app_dir;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Asset;

pub fn init_bin(){
    copy_bin("sslocal");
    copy_bin("ssservice");
    copy_bin("ssurl");
}


#[cfg(target_os = "linux")]
fn copy_bin(bin_name:&str){
    use std::os::unix::fs::PermissionsExt;

    let path = format!("linux/{}", bin_name);
    let file = Asset::get(&path).unwrap();
    let cow = file.data;
    let app_dir = get_app_dir();
    let path = app_dir.join(format!("{}",bin_name));
    if path.exists() {
        fs::remove_file(&path).unwrap();
    }
    fs::write(&path, cow).unwrap();
    let mut permissions = fs::metadata(&path).unwrap().permissions();
    permissions.set_mode(0o555);
    fs::set_permissions(&path,permissions).unwrap();
}