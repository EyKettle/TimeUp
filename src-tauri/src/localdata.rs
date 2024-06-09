use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::{from_value, to_value};
use std::fs::{self};
use std::sync::{Arc, Mutex};
use tauri::api::version;
use tauri::utils::config;
use toml::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub version: String,
    pub colormode: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tasks {
    pub title: String,
    pub items: Vec<Task>,
}

#[derive(Debug)]
pub struct WindowState {
    pub effect: String,
    pub darkmode: bool,
}

#[derive(Debug)]
struct LocalPathes {
    pub config_path: String,
    pub file_path: String,
}

lazy_static! {
    pub static ref WINDOW_STATE: Arc<Mutex<WindowState>> = Arc::new(Mutex::new(WindowState {
        effect: "None".to_string(),
        darkmode: false,
    }));
    static ref LOCAL_PATHS: Arc<LocalPathes> = Arc::new(LocalPathes {
        config_path: "timeup.config.toml".to_string(),
        file_path: "timeup.tasks.txt".to_string(),
    });
    pub static ref CONFIGS: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config {
        version: String::new(),
        colormode: String::new(),
    }));
    pub static ref TASKS: Arc<Mutex<Tasks>> = Arc::new(Mutex::new(Tasks {
        title: String::new(),
        items: Vec::new(),
    }));
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StaskStatus {
    Wait,
    Completed,
    NoNeed,
}
impl std::fmt::Display for StaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                StaskStatus::Completed => "Completed",
                StaskStatus::NoNeed => "NoNeed",
                StaskStatus::Wait => "Wait",
            }
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    completed: Option<StaskStatus>,
    description: String,
    outdate: Option<bool>,
}

pub fn assign_state(value: &WindowState) {
    let mut target = WINDOW_STATE.lock().unwrap();
    target.effect = value.effect.clone();
    target.darkmode = value.darkmode.clone();
}
fn assign_config(value: &Config) {
    let mut target = CONFIGS.lock().unwrap();
    target.version = value.version.clone();
    target.colormode = value.colormode.clone();
}

pub fn config_save() -> Result<(), String> {
    match toml::to_string(&CONFIGS.lock().unwrap().clone()) {
        Ok(contents) => match fs::write(&LOCAL_PATHS.config_path, contents) {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("[ERROR] Failed to write configs, error info: {}", e);
                Err(e.to_string())
            }
        },
        Err(e) => {
            println!("[ERROR] Failed to serialize configs, error info: {}", e);
            Err(e.to_string())
        }
    }
}
pub fn config_load() -> Result<Config, String> {
    match fs::read_to_string(&LOCAL_PATHS.config_path) {
        Ok(localtext) => {
            match localtext.parse::<Value>() {
                Ok(mut localdata) => match localdata.as_table_mut() {
                    Some(localtable) => {
                        if localtable.contains_key("version") {
                            match localtable.get("version") {
                                Some(version_key) => {
                                    match version_key.as_str() {
                                        Some(version) => {
                                            let mut configs = CONFIGS.lock().unwrap();
                                            println!(
                                            "[TEST] localtable-version is: {}; program version is: {}",
                                            version,
                                            &configs.version,
                                        );
                                            match version::compare(&configs.version, version) {
                                                Ok(result) => match result {
                                                    0 => match toml::from_str::<Config>(&localtext)
                                                    {
                                                        Ok(config) => {
                                                            println!("[INFO] Loaded local config.");
                                                            drop(configs);
                                                            assign_config(&config);
                                                            Ok(config)
                                                        }
                                                        Err(e) => {
                                                            println!(
                                                            "[ERROR] Failed to deserialize configs, error info: {}",
                                                            e
                                                        );
                                                            Err(e.to_string())
                                                        }
                                                    },
                                                    1 => {
                                                        println!(
                                                        "[WARN] This program is outdated. Config version is \"{}\" but this version is \"{}\".",
                                                        version,
                                                        &configs.version,
                                                    );
                                                        Err("Outdate version".to_string())
                                                    }
                                                    -1 => {
                                                        let currentdata = configs.clone();
                                                        match toml::Value::try_from(currentdata) {
                                                            Ok(currentvalue) => {
                                                                let currenttable = currentvalue
                                                                    .as_table()
                                                                    .unwrap();
                                                                // And don't forget to check value in future.

                                                                for (key, value) in currenttable {
                                                                    if !localtable.contains_key(key)
                                                                    {
                                                                        localtable.insert(
                                                                            key.clone(),
                                                                            value.clone(),
                                                                        );
                                                                    }
                                                                }
                                                                match to_value(localtable) {
                                                                    Ok(value) => {
                                                                        match from_value::<Config>(
                                                                            value,
                                                                        ) {
                                                                            Ok(mut result) => {
                                                                                result.version = configs.version.clone();
                                                                                println!("[INFO] Loaded and updated local config.");
                                                                                drop(configs);
                                                                                assign_config(
                                                                                    &result,
                                                                                );
                                                                                match config_save() {
                                                                                    Ok(()) => Ok(result),
                                                                                    Err(_) => Err("Save failed".to_string()),
                                                                                }
                                                                            }
                                                                            Err(e) => {
                                                                                println!("[ERROR] Failed to parse Value(serde) for building struct, error info: {e}");
                                                                                Err("Parse failed"
                                                                                    .to_string())
                                                                            }
                                                                        }
                                                                    }
                                                                    Err(e) => {
                                                                        println!(
                                                                        "[ERROR] Failed to convert table to Value(serde), error info: {e}"
                                                                    );
                                                                        Err("Convert failed"
                                                                            .to_string())
                                                                    }
                                                                }
                                                            }
                                                            Err(e) => {
                                                                println!(
                                                                "[ERROR] Failed to update local data, error info: {e}"
                                                            );
                                                                Err("Update failed".to_string())
                                                            }
                                                        }
                                                    }
                                                    _ => {
                                                        println!("[ERROR] Imposible case when compare versions.");
                                                        Err("Imposible case".to_string())
                                                    }
                                                },
                                                Err(e) => {
                                                    println!("[ERROR] Cannot compare with the version in the local config, error info: {}", e);
                                                    Err(e.to_string())
                                                }
                                            }
                                        }
                                        None => {
                                            println!("[ERROR] Cannot parse Value(toml).");
                                            Err("Parse failed".to_string())
                                        }
                                    }
                                }
                                None => {
                                    println!("[ERROR] Cannot get value of key version.");
                                    Err("Cannot get value".to_string())
                                }
                            }
                        } else {
                            println!("[ERROR] Corrupted data file.");
                            Err("Corrupted file".to_string())
                        }
                    }
                    None => {
                        println!("[ERROR] Invalid local data.");
                        Err("Invalid data".to_string())
                    }
                },
                Err(e) => {
                    println!("[ERROR] Failed to parse localtext, error info: {}", e);
                    Err(e.to_string())
                }
            }
        }
        Err(e) => {
            println!("[ERROR] Failed to read configs, error info: {}", e);
            Err(e.to_string())
        }
    }
}

// pub fn test_read() -> Option<(String, Vec<Task>)> {
//     // if file exists
//     match File::open(FILE_PATH.get().unwrap()) {
//         Ok(mut file) => {
//             // read file contents
//             let mut contents = String::new();
//             match file.read_to_string(&mut contents) {
//                 Ok(_) => {
//                     println!("Read file contents:\n{}", contents);
//                 }
//                 Err(e) => {
//                     println!("Error reading file: {}", e);
//                 }
//             }
//             test_analyze(contents.lines());
//             unsafe { Some((TITLE.clone(), TASKS.clone())) }
//         }
//         Err(e) => {
//             println!("Error opening file: {}", e);
//             None
//         }
//     }
// }

// fn test_analyze(lines: std::str::Lines) {
//     // clear last data
//     unsafe {
//         TITLE = "任务列表".to_string();
//         TASKS.clear();
//     }
//     // read Lines
//     for line in lines {
//         // get title
//         if line.starts_with("【") && line.ends_with("】") {
//             unsafe { TITLE = line.replace("【", "").replace("】", "") };
//         }
//         // get task
//         else if line.len() > 4 {
//             let (finish, des) = if line.starts_with("[ ] ") {
//                 (
//                     Some(StaskStatus::Wait),
//                     line.replace("[ ] ", "").to_string(),
//                 )
//             } else if line.starts_with("[@] ") {
//                 (
//                     Some(StaskStatus::Completed),
//                     line.replace("[@] ", "").to_string(),
//                 )
//             } else if line.starts_with("[-] ") {
//                 (
//                     Some(StaskStatus::NoNeed),
//                     line.replace("[-] ", "").to_string(),
//                 )
//             } else {
//                 (None, String::new())
//             };
//             unsafe {
//                 TASKS.push(Task {
//                     completed: finish,
//                     description: des,
//                     outdate: Some(false),
//                 })
//             };
//         }
//     }
// }

// pub fn save(title: String, tasks: Vec<Task>) -> Option<(bool, String)> {
//     unsafe {
//         TITLE = title;
//         TASKS = tasks;
//     }
//     let mut contents = String::new();
//     contents.push_str(&format!("【{}】\n", unsafe { &TITLE }));
//     for task in unsafe { &TASKS } {
//         contents.push_str(&format!(
//             "[{}] {}\n",
//             if task.completed == Some(StaskStatus::Completed) {
//                 "@"
//             } else {
//                 " "
//             },
//             task.description
//         ));
//     }
//     // if file exists
//     match File::create(FILE_PATH.get().unwrap()) {
//         Ok(mut file) => match file.write_all(contents.as_bytes()) {
//             Ok(_) => {
//                 println!("Save:\n{}", contents);
//                 Some((true, contents))
//             }
//             Err(e) => {
//                 println!("Error saving file: {}", e);
//                 Some((false, e.to_string()))
//             }
//         },
//         Err(e) => {
//             println!("Error opening file: {}", e);
//             // create file and save
//             match File::create(FILE_PATH.get().unwrap()) {
//                 Ok(mut file) => match file.write_all(contents.as_bytes()) {
//                     Ok(_) => {
//                         println!("Create file and save:\n{}", contents);
//                         Some((true, contents))
//                     }
//                     Err(e) => {
//                         println!("Error creating file: {}", e);
//                         Some((false, e.to_string()))
//                     }
//                 },
//                 Err(e) => {
//                     println!("Error creating file: {}", e);
//                     Some((false, e.to_string()))
//                 }
//             }
//         }
//     }
// }

// pub fn config_check() -> Result<(), String> {
//     match toml::to_string(unsafe { &CONFIGS }) {
//         Ok(toml) => {
//             unsafe {
//                 CONFIG_TOML = toml;
//             }
//             Ok(())
//         }
//         Err(e) => {
//             println!("[ERROR] Cannot convert config: {}", e);
//             Err(e.to_string())
//         }
//     }
// }

// pub fn config_save() -> Result<(), String> {
//     match config_check() {
//         Ok(()) => match write(unsafe { &CONFIG_PATH }, unsafe { &CONFIG_TOML }) {
//             Ok(_) => Ok(()),
//             Err(e) => {
//                 println!("[ERROR] Cannot save config: {}", e);
//                 Err(e.to_string())
//             }
//         },
//         Err(e) => Err(e),
//     }
// }

// pub fn config_load() -> Result<String, String> {
//     if let Ok(metadata) = metadata(unsafe { &CONFIG_PATH }) {
//         if metadata.is_file() {
//             if let Ok(content) = read_to_string(unsafe { &CONFIG_PATH }) {
//                 let _ = match toml::from_str::<Config>(&content.as_str()) {
//                     Ok(_) => Ok(()),
//                     Err(_) => match config_save() {
//                         Ok(()) => Ok(()),
//                         Err(e) => Err(e),
//                     },
//                 };
//             }
//         } else {
//             let _ = match config_save() {
//                 Ok(_) => Ok(()),
//                 Err(e) => Err(e),
//             };
//         }
//     }
//     match read_to_string(unsafe { &CONFIG_PATH }) {
//         Ok(content) => {
//             println!("[INFO] File content: \n{}", content);
//             match toml::from_str(&content.as_str()) {
//                 Ok(r) => {
//                     unsafe { CONFIGS = r };
//                     Ok(content)
//                 }
//                 Err(e) => {
//                     println!("[ERROR] Cannot decode config: {}", e);
//                     Err(e.to_string())
//                 }
//             }
//         }
//         Err(e) => {
//             println!("[ERROR] Cannot load config: {}", e);
//             Err(e.to_string())
//         }
//     }
// }
