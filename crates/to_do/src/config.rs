//! Loads, Saves and holds the current loaded configuration
use std::{
    fs::{self, File},
    io::Write,
    process,
};

use serde_json::Value;

/// Stores the App configuration parameters
#[derive(Debug)]
pub struct Config {
    pub settings: Value,
}

impl Config {
    /// Creates a new config struct
    pub fn new() -> Config {
        Config {
            settings: Value::Null,
        }
    }

    /// Loads the config from the config file
    pub fn load(&mut self) {
        let filepath = "config.json";

        let contents = match fs::read_to_string(filepath) {
            Ok(text) => text,
            Err(e) => {
                eprintln!("An error ocurred while parsing config.json: {e}");
                process::exit(1);
            }
        };

        self.settings = match serde_json::from_str(&contents.as_str()) {
            Ok(json) => json,
            Err(e) => {
                eprintln!("An error occured while parsing config.json: {e}");
                process::exit(1);
            }
        };
    }

    /// Saves the config to the config file
    pub fn save(&self) {
        let filepath = "config.json";

        let mut file = match File::create(filepath) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("An error occured while creating config.json: {e}");
                process::exit(1);
            }
        };

        let parsed_settings = match serde_json::to_string_pretty(&self.settings) {
            Ok(text) => text,
            Err(e) => {
                eprintln!("An error occured while parsing settings: {e}");
                process::exit(1);
            }
        };

        match file.write(&parsed_settings.as_bytes()) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("An error occured while writing config.json: {e}");
                process::exit(1);
            }
        };
    }
}
