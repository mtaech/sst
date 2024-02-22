use tauri::api::path::app_log_dir;
use crate::logger::init_logger;

pub fn setup_handler(app: &mut tauri::App)-> Result<(), Box<dyn std::error::Error + 'static>> {
    let arc = app.config().clone();
    let log_dir = app_log_dir(&arc);
    if log_dir.is_some() {
        init_logger(log_dir.unwrap()).expect("init logger error");
    }
   Ok(())
}