// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, Theme, Window};
use window_shadows::set_shadow;
use window_vibrancy::{apply_acrylic, apply_mica};
mod localdata;

// static mut app_handle: AppHandle = app;

#[tauri::command]
fn tauri_println(msg: String) {
    println!("{msg}");
}

#[tauri::command]
fn window_minimize(window: Window) -> bool {
    match window.minimize() {
        Ok(()) => {
            println!("[INFO] Window minimized.");
            true
        }
        Err(e) => {
            println!("[ERROR] Cannot minimize window. Error Info: {}", e);
            false
        }
    };
    false
}
#[tauri::command]
fn window_close(window: Window) -> bool {
    match window.close() {
        Ok(()) => {
            println!("[INFO] Window closed.");
            true
        }
        Err(e) => {
            println!("[ERROR] Cannot close window. Error Info: {}", e);
            false
        }
    };
    false
}

#[tauri::command]
fn colormode_isdark(window: Window) -> bool {
    let result = window.theme().unwrap_or_else(|err| {
        println!("[INFO] Cannot get window theme: {}", err);
        Theme::Light
    }) == Theme::Dark;
    println!("[INFO] Dark: {}", result);
    result
}

#[tauri::command]
fn colormode_change(isdark: bool, window: Window) -> bool {
    match apply_mica(&window, Some(isdark)) {
        Ok(()) => {
            println!("[INFO] Changed colormode (Mica).");
            true
        }
        Err(_) => match apply_acrylic(&window, None) {
            Ok(()) => {
                println!("[INFO] Changed colormode (Acrylic).");
                true
            }
            Err(e) => {
                println!(
                    "[ERROR] Failed to apply any window effect. Error info: {}",
                    e
                );
                false
            }
        },
    }
}

#[tauri::command]
fn config_save() -> Result<(), String> {
    match localdata::config_save() {
        Ok(()) => Ok(()),
        Err(e) => {
            println!("[ERROR] Cannot save config: {}", e);
            Err(e)
        }
    }
}

#[tauri::command]
fn config_load() -> Result<localdata::Config, String> {
    match localdata::config_load() {
        Ok(_) => {
            let result = unsafe { localdata::CONFIGS.clone() };
            Ok(result)
        }
        Err(e) => {
            println!("[ERROR] Cannot save config: {}", e);
            Err(e)
        }
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            localdata::init();
            let window = app.get_window("main").unwrap();
            let dark = window.theme().unwrap_or_else(|err| {
                println!("Cannot get window theme: {}", err);
                Theme::Light
            }) == Theme::Dark;
            set_shadow(&window, true).expect("Unsupported Platform!");
            match apply_mica(&window, Some(dark)) {
                Ok(()) => println!("Successfully applied Mica."),
                Err(e) => {
                    println!(
                        "Failed to apply Mica, try to apply Acrylic. Error info: {}",
                        e
                    );
                    match apply_acrylic(&window, None) {
                        Ok(()) => println!("Successfully applied Acrylic."),
                        Err(e) => println!("Failed to apply any window effect. Error info: {}", e),
                    }
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            tauri_println,
            window_minimize,
            window_close,
            colormode_isdark,
            colormode_change,
            config_load,
            config_save,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
