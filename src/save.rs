use std::fs::{
    File, remove_file
};
use std::path::PathBuf;
use std::io::{Read, Write};

use dirs::home_dir;
use bincode::{serialize, deserialize};
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
    let data = serialize(&data).expect("Could not serialize data");
    file.write_all(&data).expect("Could not write serialized data");
}

pub fn read_or<T: DeserializeOwned + Clone>(default: T) -> T {
    let path = data_path();

    if path.exists() {
        let mut file = File::open(path).expect("Could not open file to deserialize from");
        let mut buffer = Vec::<u8>::new();
        file.read_to_end(&mut buffer).expect("Could not read data from file");
        let deserialized: T = deserialize(&buffer[..]).expect("Could not deserialize data");
        deserialized.clone()
    } else {
        default
    }
}
