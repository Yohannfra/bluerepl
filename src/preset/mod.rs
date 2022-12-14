use crate::controllers;
use crate::repl::commands;
use serde::Deserialize;
use std::collections::HashMap;

use std::error::Error;

use std::{fs, path};

mod getters;
mod print;
mod run;
mod verify;

#[derive(Deserialize, Debug)]
pub struct Preset {
    #[serde(skip_deserializing)]
    fp: path::PathBuf,
    pub device: Option<Device>,
    services: Option<HashMap<String, Service>>,
    commands: Option<HashMap<String, Command>>,
    functions: Option<HashMap<String, Function>>,
}

#[derive(Deserialize, Debug)]
pub struct Device {
    name: Option<String>,
    address: Option<String>,
    #[serde(skip_deserializing)]
    pub autoconnect: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct Service {
    uuid: String,
    characteristics: Option<HashMap<String, Characteristic>>,
}

#[derive(Deserialize, Debug)]
pub struct Characteristic {
    uuid: String,
}

fn format_default() -> String {
    "hex".to_string()
}

#[derive(Deserialize, Debug)]
pub struct Command {
    command_type: String,
    service: String,
    characteristic: String,
    payload: Option<String>,

    #[serde(default = "format_default")]
    format: String,
}

#[derive(Deserialize, Debug)]
pub struct Function {
    commands_delay_ms: Vec<u64>,
    commands: Vec<String>,
}

impl Preset {
    fn parse_file(fp: path::PathBuf) -> Result<Preset, String> {
        let contents = match fs::read_to_string(&fp) {
            Ok(c) => c,
            Err(_) => {
                return Err(format!("Could not read file: '{}'", fp.to_string_lossy()));
            }
        };

        let mut pr: Preset = match toml::from_str(&contents) {
            Ok(d) => d,
            Err(e) => {
                return Err(format!(
                    "Unable to load data from: {:?}: {}",
                    fp.to_string_lossy(),
                    e
                ))
            }
        };
        pr.fp = fp;
        Ok(pr)
    }

    pub fn new(fp: std::path::PathBuf) -> Result<Preset, String> {
        println!("Loading {}", fp.to_string_lossy());

        let pr = Self::parse_file(fp)?;
        pr.verify();

        Ok(pr)
    }

    pub fn should_autoconnect(&self) -> bool {
        if let Some(device) = &self.device {
            return device.autoconnect.unwrap_or(false);
        }
        false
    }

    pub async fn autoconnect(
        &self,
        bt: &mut dyn controllers::BleController,
    ) -> Result<(), Box<dyn Error>> {
        commands::scan::run(bt, 5, false, false).await.unwrap();
        if let Some(name) = &self.device.as_ref().unwrap().name {
            commands::connect::by_name(bt, name).await?;
        } else {
            commands::connect::by_address(
                bt,
                self.device.as_ref().unwrap().address.as_ref().unwrap(),
            )
            .await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_all_base_presets() {
        let test_files: [&str; 5] = [
            "presets/battery.toml",
            "presets/nus.toml",
            "presets/color.toml",
            "presets/hrs.toml",
            "presets/neopixel_controller.toml",
        ];

        for fp in test_files {
            let preset = Preset::new(path::PathBuf::from(fp));
            assert!(preset.is_ok(), "{:?}", preset);
        }
    }
}
