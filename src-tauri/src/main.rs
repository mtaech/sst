// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::logger::init_logger;
use crate::setup::setup_handler;

mod handlers;
mod model;
mod logger;
mod setup;

fn main() {

    tauri::Builder::default()
        .setup(setup_handler)
        .invoke_handler(tauri::generate_handler![handlers::get_profile_list,handlers::add_profile,handlers::exe_server])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
