use log::info;
use tauri::api::process::{Command, CommandEvent};
use sst_core::config::get_config;
use sst_core::model::{Profile, Server};
use sst_core::utils::get_app_dir;

use crate::model::Resp;

#[tauri::command]
pub fn get_profile_list()->Resp<Vec<Profile>>{
    let config = get_config();
    let profile = config.profiles;
    Resp::from_data(Some(profile))
}

#[tauri::command]
pub fn add_profile(url:String)->Resp<String>{
    sst_core::config::add_profile(url);
    Resp::from_msg(Some("success".to_string()))
}

#[tauri::command]
pub fn exe_server(server:Server)->Resp<String>{
    println!("server {:?}",&server);
    match server.ss_url {
        None => {return  Resp::from_msg(Some("success".to_string())) }
        Some(ss_url) => {
            println!("ss url:{}",ss_url);
            let (mut rx, mut child) = Command::new_sidecar("sslocal")
                .expect("side car error")
                .args(["-b","127.0.0.1:8080","--server-url",&ss_url])
                .spawn()
                .expect("Failed to spawn sidecar");

            tauri::async_runtime::spawn(async move {
                // read events such as stdout
                while let Some(event) = rx.recv().await {
                    match event {
                        CommandEvent::Stderr(line) => {
                            info!("stderr {:?}",line);
                            child.write("message from Rust\n".as_bytes()).unwrap();
                        }
                        CommandEvent::Stdout(line) => {
                            info!("out {:?}",line);
                            child.write("message from Rust\n".as_bytes()).unwrap();
                        }
                        CommandEvent::Error(line) => {
                            info!("error {:?}",line);
                            child.write("message from Rust\n".as_bytes()).unwrap();
                        }
                        CommandEvent::Terminated(payload) => {
                            info!("Terminated {:?}", payload);
                            child.write("message from Rust\n".as_bytes()).unwrap();
                        }
                        _ => {}
                    }

                }
            });
            Resp::from_msg(Some("success".to_string()))
        }
    }

}