// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::utils::set_window_shadow;
use tauri::Manager;
use window_vibrancy::apply_acrylic;
mod utils;
mod localdata;
mod prodata;

#[tauri::command]
fn datatest_read() -> Option<(String, Vec<localdata::Task>)> {
    localdata::test_read()
}

#[tauri::command]
fn datatest_save(title: String, tasks: Vec<localdata::Task>) -> Option<(bool, String)> {
    localdata::save(title, tasks)
}

#[tauri::command]
fn prodata_read() -> Option<prodata::ProData> {
    prodata::read()
}

#[tauri::command]
fn prodata_save(configs: Option<prodata::ProData>) -> Option<(bool, String)> {
    prodata::save(configs)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            set_window_shadow(app);
            apply_acrylic(app.get_window("customization").unwrap(), None).expect("Unsupported platform! 'apply_blur' is only supported on Windows");
            localdata::init();
            prodata::init();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![datatest_read, datatest_save, prodata_read, prodata_save])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}