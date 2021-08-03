mod v0;
mod v1;
mod v2;

use std::fs::{
    File, remove_file
};
use std::path::PathBuf;
use std::io::{Read, Write};

use dirs::home_dir;

use crate::AppData;
use v0::{V0AppData, upgrade_v0_to_v1};
use v1::{V1AppData, upgrade_v1_to_v2};
use v2::{V2AppData, upgrade_v2_to_current};

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
    serde_json::from_str::<AppData>(json).unwrap_or_else(|_| {
        let v2_app_data = serde_json::from_str::<V2AppData>(json).unwrap_or_else(|_| {
            let v1_app_data = serde_json::from_str::<V1AppData>(json).unwrap_or_else(|_| {
                let v0_app_data = serde_json::from_str::<V0AppData>(json).expect("Invalid save format");
                upgrade_v0_to_v1(v0_app_data)
            });
            upgrade_v1_to_v2(v1_app_data)
        });
        upgrade_v2_to_current(v2_app_data)
    })
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
