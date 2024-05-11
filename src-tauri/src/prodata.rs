use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::prelude::*;
use std::sync::OnceLock;

static FILE_PATH: OnceLock<std::path::PathBuf> = OnceLock::new();
static mut DATAS: Option<ProData> = None;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProData {
    colormode: String,
}

pub fn init() {
    let _ = FILE_PATH.set("timeup.json".into());
}

pub fn read() -> Option<ProData> {
    // if file exists
    match File::open(FILE_PATH.get().unwrap()) {
        Ok(mut file) => {
            // read file contents
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Ok(_) => {
                    println!("Read file contents:\n{}", contents);
                    let datas = serde_json::from_str(&contents).unwrap_or_else(|err| {
                        println!("Error loading json: {}", err);
                        ProData {
                            colormode: "auto".to_string(),
                        }
                    });
                    unsafe {
                        DATAS = Some(datas);
                        Some(DATAS.clone()).unwrap()
                    }
                }
                Err(e) => {
                    println!("Error reading file: {}", e);
                    None
                }
            }
        }
        Err(e) => {
            println!("Error opening file: {}", e);
            None
        }
    }
}

pub fn save(configs: Option<ProData>) -> Option<(bool, String)> {
    println!("{:?}", configs);
    unsafe {
        DATAS = configs;
        println!("{:?}", DATAS);
    }
    let contents = serde_json::to_string(unsafe { &DATAS }).unwrap();
    // if file exists
    match File::create(FILE_PATH.get().unwrap()) {
        Ok(mut file) => match file.write_all(contents.as_bytes()) {
            Ok(_) => {
                println!(
                    "Save:\n{}; {}",
                    contents,
                    FILE_PATH.get().unwrap().display()
                );
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
