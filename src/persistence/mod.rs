mod v0;

use std::fs::{
    File, remove_file
};
use std::path::PathBuf;
use std::io::{Read, Write};

use dirs::home_dir;

use crate::AppData;
use v0::{V0AppData, upgrade};

fn data_path() -> PathBuf {
    let mut path = home_dir().expect("Could not read home directory");
    path.push("todo.pando");
    path
}

pub fn save(data: AppData) {
    let path = data_path();

    if path.exists() {
        remove_file(&path).expect("Could not delete previous save");
    }

    let mut file = File::create(path).expect("Could not create file to serialize to");
    let data = serde_json::to_string(&data).expect("Could not serialize data");
    file.write_all(data.as_bytes()).expect("Could not write serialized data");
}

pub fn deserialize(json: &str) -> AppData {
    if let Ok(app_data) = serde_json::from_str::<AppData>(json) {
        app_data
    } else if let Ok(v0_app_data) = serde_json::from_str::<V0AppData>(json) {
        upgrade(v0_app_data)
    } else {
        panic!("Could not deserialize save");
    }
}

pub fn read_or(default: AppData) -> AppData {
    let path = data_path();

    if path.exists() {
        let mut file = File::open(path).expect("Could not open file to deserialize from");
        let mut json = String::new();
        file.read_to_string(&mut json).expect("Could not read data from file");
        deserialize(&json)
    } else {
        default
    }
}
