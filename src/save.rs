use std::fs::{
    File, remove_file
};
use std::path::PathBuf;
use std::io::{Read, Write};

use dirs::home_dir;
use serde::Serialize;
use serde::de::DeserializeOwned;

fn data_path() -> PathBuf {
    let mut path = home_dir().expect("Could not read home directory");
    path.push("todo.pando");
    path
}

pub fn save<T: Serialize>(data: T) {
    let path = data_path();

    if path.exists() {
        remove_file(&path).expect("Could not delete previous save");
    }

    let mut file = File::create(path).expect("Could not create file to serialize to");
    let data = serde_json::to_string(&data).expect("Could not serialize data");
    file.write_all(data.as_bytes()).expect("Could not write serialized data");
}

pub fn read_or<T: Serialize + DeserializeOwned + Clone>(default: T) -> T {
    return default;

    let path = data_path();

    if path.exists() {
        let mut file = File::open(path).expect("Could not open file to deserialize from");
        let mut json = String::new();
        file.read_to_string(&mut json).expect("Could not read data from file");
        let deserialized: T = serde_json::from_str(&json).expect("Could not deserialize data");
        deserialized.clone()
    } else {
        default
    }
}
