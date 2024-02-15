// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::utils::set_window_shadow;
use tauri::Manager;
use window_vibrancy::apply_mica;
mod utils;
mod localdata;

#[tauri::command]
fn datatest_read() -> Option<(String, Vec<localdata::Task>)> {
    localdata::test_read()
}

#[tauri::command]
fn datatest_save(title: String, tasks: Vec<localdata::Task>) -> Option<(bool, String)> {
    localdata::save(title, tasks)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            set_window_shadow(app);
            apply_mica(app.get_window("customization").unwrap(), None).expect("Unsupported platform! 'apply_blur' is only supported on Windows");
            localdata::init();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![datatest_read, datatest_save])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}