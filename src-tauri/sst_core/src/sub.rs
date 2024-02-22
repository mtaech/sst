use std::fs;
use std::process::{Command, id};

use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use log::info;
use crate::model::{Server, Sub};

use crate::utils::{get_app_dir, get_config_dir};

pub fn download_sub(url:&str)->String{
    let response = reqwest::blocking::get(url).unwrap();
   return response.text_with_charset("utf-8").unwrap();
}

pub fn get_sub(sub_str:String) ->Sub{
    info!("sub str: {}",sub_str);
    let decode_vec = BASE64_STANDARD.decode(sub_str).unwrap();
    let decode_str = String::from_utf8(decode_vec).unwrap();
    let mut sub = Sub::default();
    for line in decode_str.lines() {
        info!("line {}",line);
        if line.starts_with("ss://") {
            let server = handle_server(line);
            if server.is_some() {
                sub.servers.push(server.unwrap());
            }
        }
        if line.starts_with("STATUS") {
            let status = handle_sub_info(line);
            sub.status = Some(status);
        }
        if line.starts_with("REMARKS") {
            let status = handle_sub_info(line);
            sub.remarks = Some(status);
        }
    }
    return sub;
}

fn handle_server(line:&str) ->Option<Server>{
    let ssurl_path = get_app_dir().join("ssurl-x86_64-unknown-linux-gnu-x86_64-unknown-linux-gnu ");
    info!("ssurl-x86_64-unknown-linux-gnu path {:?}",&ssurl_path);
    let mut command = Command::new(&ssurl_path);
    command.args(&["--decode",line]);
    let output = command.output().unwrap();
    if output.status.success() {
        let string = String::from_utf8(output.stdout).unwrap();
        let mut server:Server = serde_json::from_str(&string).unwrap();
        let name = handle_name(line);
        server.name = Some(name);
        info!("output : {:?}",&server);
        return Some(server);
    }
    None
}
fn handle_name(line:&str)->String{
    let idx = line.rfind("#").unwrap();
    let name = utf8_slice::slice(line,idx+1,line.len());
    return urlencoding::decode(name).unwrap().to_string();
}

fn handle_sub_info(line:&str) -> String {
    let idx = line.find("=").unwrap();
    return  String::from(utf8_slice::slice(line,idx+1,line.len()));
}

