use std::fmt::Debug;

use chrono::Local;
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug,Default)]
pub struct Config{
    pub profiles:Vec<Profile>,
    pub local_port:u32
}

impl Config{
    pub fn default()->Self{
        Self{
            local_port:1080,
            profiles:vec![]
        }
    }
}

#[derive(Serialize,Deserialize,Debug,Default)]
pub struct Profile{
    pub name:String,
    pub sub_url:String,
    pub sub:Sub,
}

impl Profile{
    pub fn from_sub(sub:Sub,url:String)->Profile{
        let option = &sub.remarks;
        let mut name = "".to_string();
        if option.is_some() {
            name = option.clone().unwrap();
        }else {
            name = format!("config_{:?}",Local::now().format("%Y-%m-%d_%H:%M:%S"));
        }
        Self{
            name,
            sub_url: url,
            sub,
        }
    }
}

#[derive(Serialize,Deserialize,Debug,Default)]
pub struct Sub{
    pub remarks:Option<String>,
    pub status:Option<String>,
    pub servers:Vec<Server>,
}

impl Sub {
    pub fn default()->Self{
        Self{
            ..Default::default()
        }
    }
}

#[derive(Serialize,Deserialize,Debug,Default)]
pub struct Server {
    pub name:Option<String>,
    pub server:String,
    pub server_port:u32,
    pub password:Option<String>,
    pub method:Option<String>,
    pub mode:Option<String>,
    pub ss_url:Option<String>
}