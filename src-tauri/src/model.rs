use serde::{Deserialize, Serialize};
use sst_core::model::Profile;

#[derive(Serialize,Deserialize)]
pub struct Resp<T>{
    pub code:u32,
    pub msg:Option<String>,
    pub data:Option<T>
}

impl <T> Resp<T>{
    pub fn from_data(data: Option<T>) ->Resp<T>{
        Self{
            code:200,
            msg:Some("success".to_string()),
            data
        }
    }

    pub fn from_msg(msg: Option<String>) ->Resp<T>{
        Self{
            code:200,
            msg,
            data:None
        }
    }

    pub fn error(msg:Option<String>)->Resp<T>{
        Self{
            code:200,
            msg,
            data:None
        }
    }
}