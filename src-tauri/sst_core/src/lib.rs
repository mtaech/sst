mod init;
pub mod utils;
pub mod sub;
pub mod model;
pub mod config;

use rust_embed::RustEmbed;
use crate::config::init_config;
use crate::init::init_bin;

pub fn init(){
    init_config();
    init_bin();
}

