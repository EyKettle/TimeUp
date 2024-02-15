use std::fs::File;
use std::io::prelude::*;
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

// static const for file path
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
        Ok(mut file) => {
            match file.write_all(contents.as_bytes()) {
                Ok(_) => {
                    println!("Save:\n{}", contents);
                    return Some((true, contents));
                },
                Err(e) => {
                    println!("Error saving file: {}", e);
                    return Some((false, e.to_string()));
                }
            }
        },
        Err(e) => {
            println!("Error opening file: {}", e);
            // create file and save
            match File::create(FILE_PATH.get().unwrap()) {
                Ok(mut file) => match file.write_all(contents.as_bytes()) {
                    Ok(_) => {
                        println!("Create file and save:\n{}", contents);
                        return Some((true, contents));
                    },
                    Err(e) => {
                        println!("Error creating file: {}", e);
                        return Some((false, e.to_string()));
                    },
                },
                Err(e) => {
                    println!("Error creating file: {}", e);
                    return Some((false, e.to_string()));
                },
            }
        }
    }
}
