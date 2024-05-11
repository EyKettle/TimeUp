use serde::{Deserialize, Serialize};
use std::fs::{metadata, read_to_string, write, File};
use std::io::prelude::*;
use std::sync::OnceLock;
use toml;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub version: String,
    pub colormode: String,
}

pub static mut CONFIGS: Config = Config {
    version: String::new(),
    colormode: String::new(),
};
static mut CONFIG_TOML: String = String::new();
static mut CONFIG_PATH: String = String::new();

static FILE_PATH: OnceLock<std::path::PathBuf> = OnceLock::new();

static mut TITLE: String = String::new();
static mut TASKS: Vec<Task> = Vec::new();

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

pub fn init() {
    let _ = FILE_PATH.set("timeup.tasks.txt".into());
    // let _ = FILE_PATH.set(
    //     dirs::desktop_dir()
    //         .ok_or(std::io::Error::new(
    //             std::io::ErrorKind::NotFound,
    //             "Desktop directory not found.",
    //         ))
    //         .unwrap()
    //         .join("test.txt"),
    // );
    unsafe {
        CONFIGS = Config {
            version: "0.0.1".to_string(),
            colormode: "auto".to_string(),
        };
        CONFIG_PATH = "timeup.config.toml".to_string();
    }
}

pub fn test_read() -> Option<(String, Vec<Task>)> {
    // if file exists
    match File::open(FILE_PATH.get().unwrap()) {
        Ok(mut file) => {
            // read file contents
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Ok(_) => {
                    println!("Read file contents:\n{}", contents);
                }
                Err(e) => {
                    println!("Error reading file: {}", e);
                }
            }
            test_analyze(contents.lines());
            unsafe { Some((TITLE.clone(), TASKS.clone())) }
        }
        Err(e) => {
            println!("Error opening file: {}", e);
            None
        }
    }
}

fn test_analyze(lines: std::str::Lines) {
    // clear last data
    unsafe {
        TITLE = "任务列表".to_string();
        TASKS.clear();
    }
    // read Lines
    for line in lines {
        // get title
        if line.starts_with("【") && line.ends_with("】") {
            unsafe { TITLE = line.replace("【", "").replace("】", "") };
        }
        // get task
        else if line.len() > 4 {
            let (finish, des) = if line.starts_with("[ ] ") {
                (
                    Some(StaskStatus::Wait),
                    line.replace("[ ] ", "").to_string(),
                )
            } else if line.starts_with("[@] ") {
                (
                    Some(StaskStatus::Completed),
                    line.replace("[@] ", "").to_string(),
                )
            } else if line.starts_with("[-] ") {
                (
                    Some(StaskStatus::NoNeed),
                    line.replace("[-] ", "").to_string(),
                )
            } else {
                (None, String::new())
            };
            unsafe {
                TASKS.push(Task {
                    completed: finish,
                    description: des,
                    outdate: Some(false),
                })
            };
        }
    }
}

pub fn save(title: String, tasks: Vec<Task>) -> Option<(bool, String)> {
    unsafe {
        TITLE = title;
        TASKS = tasks;
    }
    let mut contents = String::new();
    contents.push_str(&format!("【{}】\n", unsafe { &TITLE }));
    for task in unsafe { &TASKS } {
        contents.push_str(&format!(
            "[{}] {}\n",
            if task.completed == Some(StaskStatus::Completed) {
                "@"
            } else {
                " "
            },
            task.description
        ));
    }
    // if file exists
    match File::create(FILE_PATH.get().unwrap()) {
        Ok(mut file) => match file.write_all(contents.as_bytes()) {
            Ok(_) => {
                println!("Save:\n{}", contents);
                Some((true, contents))
            }
            Err(e) => {
                println!("Error saving file: {}", e);
                Some((false, e.to_string()))
            }
        },
        Err(e) => {
            println!("Error opening file: {}", e);
            // create file and save
            match File::create(FILE_PATH.get().unwrap()) {
                Ok(mut file) => match file.write_all(contents.as_bytes()) {
                    Ok(_) => {
                        println!("Create file and save:\n{}", contents);
                        Some((true, contents))
                    }
                    Err(e) => {
                        println!("Error creating file: {}", e);
                        Some((false, e.to_string()))
                    }
                },
                Err(e) => {
                    println!("Error creating file: {}", e);
                    Some((false, e.to_string()))
                }
            }
        }
    }
}

pub fn config_check() -> Result<(), String> {
    match toml::to_string(unsafe { &CONFIGS }) {
        Ok(toml) => {
            unsafe {
                CONFIG_TOML = toml;
            }
            Ok(())
        }
        Err(e) => {
            println!("[ERROR] Cannot convert config: {}", e);
            Err(e.to_string())
        }
    }
}

pub fn config_save() -> Result<(), String> {
    match config_check() {
        Ok(()) => match write(unsafe { &CONFIG_PATH }, unsafe { &CONFIG_TOML }) {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("[ERROR] Cannot save config: {}", e);
                Err(e.to_string())
            }
        },
        Err(e) => Err(e),
    }
}

pub fn config_load() -> Result<String, String> {
    if let Ok(metadata) = metadata(unsafe { &CONFIG_PATH }) {
        if metadata.is_file() {
            if let Ok(content) = read_to_string(unsafe { &CONFIG_PATH }) {
                let _ = match toml::from_str::<Config>(&content.as_str()) {
                    Ok(_) => Ok(()),
                    Err(_) => match config_save() {
                        Ok(()) => Ok(()),
                        Err(e) => Err(e),
                    },
                };
            }
        } else {
            let _ = match config_save() {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            };
        }
    }
    match read_to_string(unsafe { &CONFIG_PATH }) {
        Ok(content) => {
            println!("[INFO] File content: \n{}", content);
            match toml::from_str(&content.as_str()) {
                Ok(r) => {
                    unsafe { CONFIGS = r };
                    Ok(content)
                }
                Err(e) => {
                    println!("[ERROR] Cannot decode config: {}", e);
                    Err(e.to_string())
                }
            }
        }
        Err(e) => {
            println!("[ERROR] Cannot load config: {}", e);
            Err(e.to_string())
        }
    }
}
