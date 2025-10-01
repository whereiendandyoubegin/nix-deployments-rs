use crate::types::{ DesiredState, DeployedState, Result, AppError };
use std::io::BufReader;
use std::path::Path;
use std::fs::{self, File};
use serde_json::Value


fn load_json(path: &str) -> Value {
    let file = File::open(path)
        .expect("Please ensure definitions/config.json is present, exiting.");
    let file_read = BufReader::new(file);
    serde_json::from_reader(file_read)
        .expect("JSON found but not successfully parsed, may be malformed")
}

pub fn main() {
    let config_file = fs::read_to_string("definitions/config.json");
    println!("{:#?}", config_file);
}
