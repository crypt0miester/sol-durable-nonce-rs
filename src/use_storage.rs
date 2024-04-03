use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use lazy_static::lazy_static;


lazy_static! {
    pub static ref NONCE_FILE: Option<String> = {
        dirs_next::home_dir().map(|mut path| {
            path.extend([".config", "solana", "durable_nonce_file.json"]);
            path.to_str().unwrap().to_string()
        })
    };
}

pub mod local_storage {
    use super::*;

    pub fn read_json_file<P: AsRef<Path>>(path: P) -> io::Result<HashMap<String, String>> {
        let mut file = File::open(path)?;
        let mut json_data = String::new();
        file.read_to_string(&mut json_data)?;
        serde_json::from_str(&json_data).map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
    }

    pub fn write_json_file<P: AsRef<Path>>(path: P, data: &HashMap<String, String>) -> io::Result<()> {
        let json_data = serde_json::to_string(data)?;
        std::fs::write(path, json_data)
    }

    pub fn get_item(key: &str) -> Option<String> {

        let file_path = NONCE_FILE.as_ref().unwrap();
        if let Ok(storage) = read_json_file(&file_path) {
            storage.get(key).cloned()
        } else {
            None
        }
    }

    pub fn set_item(key: &str, value: &str) -> bool {
        let file_path = NONCE_FILE.as_ref().unwrap();
        let mut storage = match read_json_file(&file_path) {
            Ok(map) => map,
            Err(_) => HashMap::new(),
        };
        storage.insert(key.to_string(), value.to_string());
        if let Err(err) = write_json_file(&file_path, &storage) {
            // Handle the error
            println!("Error writing to file: {}", err);
            return false;
        }
        true
    }

    pub fn remove_item(key: &str) {
        let file_path = NONCE_FILE.as_ref().unwrap();
        let mut storage = match read_json_file(&file_path) {
            Ok(map) => map,
            Err(_) => HashMap::new(),
        };
        storage.remove(key);
        if let Err(err) = write_json_file(&file_path, &storage) {
            // Handle the error
            println!("Error writing to file: {}", err);
        }
    }
}