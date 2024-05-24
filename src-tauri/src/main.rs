// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use tauri::{Manager, State, Theme, Window};
use window_shadows::set_shadow;
use window_vibrancy::{apply_acrylic, apply_mica, clear_acrylic, clear_mica, Color};
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
async fn colormode_change(isdark: bool, window: Window) -> Result<bool, bool> {
    let mut state = localdata::WINDOW_STATE.lock().unwrap();
    state.darkmode = isdark;
    println!(
        "[INFO] Changed colormode to {}{}.",
        if isdark {
            "Dark".to_string()
        } else {
            "Light".to_string()
        },
        if state.effect == "Mica".to_string() {
            " (Mica)".to_string()
        } else {
            String::new()
        }
    );
    if state.effect == "Mica".to_string() {
        match apply_mica(&window, Some(isdark)) {
            Ok(()) => {
                state.effect = "Mica".to_string();
                Ok(true)
            }
            Err(e) => {
                println!("[ERROR] Failed to apply Mica. Error info: {}", e);
                Ok(false)
            }
        }
    } else {
        Ok(false)
    }
}

#[tauri::command]
async fn windoweffect_change(ifclear: bool, window: Window) -> String {
    let mut state = localdata::WINDOW_STATE.lock().unwrap();
    if ifclear {
        let result: Result<(), window_vibrancy::Error>;
        match state.effect.as_str() {
            "Mica" => {
                result = clear_mica(&window);
            }
            "Acrylic" => {
                result = clear_acrylic(&window);
            }
            _ => {
                return "None".to_string();
            }
        }
        match result {
            Ok(()) => {
                println!("[INFO] Cleared window effect.");
                state.effect = "None".to_string();
            }
            Err(e) => {
                println!("[ERROR] Cannot clear window effect. Error info: {}", e);
            }
        };
    } else {
        match state.effect.as_str() {
            "Mica" => {
                clear_mica(&window).unwrap();
                match apply_acrylic(&window, None) {
                    Ok(()) => {
                        println!(
                            "[INFO] Changed colormode {} (Acrylic).",
                            if state.darkmode {
                                "Dark".to_string()
                            } else {
                                "Light".to_string()
                            }
                        );
                        state.effect = "Acrylic".to_string();
                    }
                    Err(e) => {
                        println!("[ERROR] Failed to apply Acrylic. Error info: {}", e);
                    }
                }
            }
            "Acrylic" => {
                clear_acrylic(&window).unwrap();
                match apply_mica(&window, Some(state.darkmode)) {
                    Ok(()) => {
                        state.effect = "Mica".to_string();
                        println!(
                            "[INFO] Changed colormode to Mica.\nStatus: {}",
                            &state.effect
                        );
                    }
                    Err(e) => {
                        println!("[ERROR] Failed to apply Mica. Error info: {}", e);
                    }
                }
            }
            _ => {}
        }
    }
    state.effect.clone()
}

#[tauri::command]
fn windoweffect_get() -> String {
    let state = localdata::WINDOW_STATE.lock().unwrap();
    let result = state.effect.clone();
    println!("[INFO] Get colormode state: {}", result);
    result
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
            let window = app.get_window("main").unwrap();
            let mut effect = "None".to_string();
            let dark = window.theme().unwrap_or_else(|err| {
                println!("Cannot get window theme: {}", err);
                Theme::Light
            }) == Theme::Dark;
            set_shadow(&window, true).expect("Unsupported Platform!");
            match apply_mica(&window, Some(dark)) {
                Ok(()) => {
                    effect = "Mica".to_string();
                    println!("Successfully applied Mica.");
                }
                Err(e) => {
                    println!(
                        "Failed to apply Mica, try to apply Acrylic. Error info: {}",
                        e
                    );
                    match apply_acrylic(&window, None) {
                        Ok(()) => {
                            effect = "Acrylic".to_string();
                            println!("Successfully applied Acrylic.");
                        }
                        Err(e) => println!("Failed to apply any window effect. Error info: {}", e),
                    }
                }
            }
            localdata::init(localdata::WindowState {
                effect,
                darkmode: dark,
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            tauri_println,
            window_minimize,
            window_close,
            colormode_isdark,
            colormode_change,
            windoweffect_get,
            windoweffect_change,
            config_load,
            config_save,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
