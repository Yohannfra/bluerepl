pub mod parser;

use serde::Deserialize;
use std::collections::HashMap;

use std::{fs, path};
use toml;

#[derive(Deserialize, Debug)]
pub struct Preset {
    device: Option<Device>,
    services: Option<HashMap<String, Vec<Service>>>,
    commands: Option<HashMap<String, Vec<Command>>>,
}

#[derive(Deserialize, Debug)]
pub struct Device {
    name: Option<String>,
    address: Option<String>,
    autoconnect: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct Service {
    name: String,
    uuid: String,
    characteristics: Option<HashMap<String, Vec<Characteristic>>>,
}

#[derive(Deserialize, Debug)]
pub struct Characteristic {
    name: String,
    uuid: String,
}


#[derive(Deserialize, Debug)]
pub struct Command {
    command_type: String,
    service: String,
    characteristic: String,
    payload: Option<Vec<String>>,
}

impl Preset {
    fn parse_file(fp: path::PathBuf) -> Result<Preset, String> {
        let contents = match fs::read_to_string(&fp) {
            Ok(c) => c,
            Err(_) => {
                return Err(format!("Could not read file: '{:?}'", &fp.as_path()));
            }
        };

        let pr: Preset = match toml::from_str(&contents) {
            Ok(d) => d,
            Err(e) => {
                return Err(format!(
                    "Unable to load data from: {:?}: {}",
                    &fp.as_path(),
                    e
                ))
            }
        };
        Ok(pr)
    }
    pub fn new(fp: std::path::PathBuf) -> Result<Preset, String> {
        Self::parse_file(fp)
    }
}
