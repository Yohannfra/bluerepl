use comfy_table::{Attribute, Cell, Table};

use crate::controllers;
use crate::repl::commands;
use serde::Deserialize;
use std::collections::HashMap;

use std::error::Error;

use std::{fs, path};
use toml;

use std::time::Duration;
use tokio::time;

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

#[derive(Deserialize, Debug)]
pub struct Command {
    command_type: String,
    service: String,
    characteristic: String,
    payload: Option<String>,
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

    fn verify(&self) {
        // check if services and characteristics typed in commands descriptions are defined in
        // the preset
        if self.commands.is_some() {
            // check services
            for cmd in self.commands.as_ref().unwrap() {
                if self.services.is_none()
                    || self.services.as_ref().unwrap().contains_key(&cmd.1.service) == false
                {
                    panic!(
                        "Service '{}' in command '{}' not found",
                        cmd.1.service, cmd.0
                    );
                }

                // check characteristics
                for ser in self.services.as_ref().unwrap() {
                    if cmd.1.service == *ser.0 {
                        if ser.1.characteristics.is_none()
                            || ser
                                .1
                                .characteristics
                                .as_ref()
                                .unwrap()
                                .contains_key(&cmd.1.characteristic)
                                == false
                        {
                            panic!(
                                "Characteristic '{}' in command '{}' not found",
                                cmd.1.characteristic, cmd.0
                            );
                        }
                    }
                }
            }
        }

        // check that if autoconnect=true there is also device name or address
        if self.device.is_some() {
            if self.device.as_ref().unwrap().autoconnect.unwrap_or(false) {
                if self.device.as_ref().unwrap().name.is_none()
                    && self.device.as_ref().unwrap().address.is_none()
                {
                    panic!("You must provide a name or an address to use the autoconnect feature");
                }
            }
        }

        // check that the function commands array is the same length as the function delay array
        if self.functions.is_some() {
            for f in self.functions.as_ref().unwrap() {
                if f.1.commands.len() != f.1.commands_delay_ms.len() {
                    panic!("In function {} 'commands' and 'commands_delay_ms' don't have the same length", f.0)
                }
            }
        }
    }

    pub fn print(&self) {
        let mut table = Table::new();

        table.set_header(vec![
            Cell::new("File name:").add_attribute(Attribute::Bold),
            Cell::new(&self.fp.to_string_lossy()),
        ]);

        // Device infos
        if self.device.is_some() {
            let device_ref = self.device.as_ref().unwrap();

            table.add_row(vec![
                "Device name\nDevice address\nDevice Autoconnect",
                &format!(
                    "{}\n{}\n{}",
                    device_ref.name.as_ref().unwrap_or(&"".to_owned()),
                    device_ref.address.as_ref().unwrap_or(&"".to_owned()),
                    device_ref.autoconnect.unwrap_or(false)
                ),
            ]);
        }

        // Service(s)
        if self.services.is_some() {
            table.add_row(vec![Cell::new("Service").add_attribute(Attribute::Bold)]);

            for (key, ser) in self.services.as_ref().unwrap() {
                let fmt_service = format!("{}\n{}", key, ser.uuid);
                let mut vec_service = vec!["Name\nUUID".to_owned(), fmt_service];

                if ser.characteristics.is_some() {
                    for (key, charac) in ser.characteristics.as_ref().unwrap() {
                        vec_service[0].push_str("\n\nCharacteristic:\n");
                        vec_service[0].push_str(" - Name:\n - UUID");
                        vec_service[1].push_str(&format!("\n\n\n{}\n{}", key, charac.uuid));
                    }
                }
                table.add_row(vec_service);
            }
        }

        // Commands
        if self.commands.is_some() {
            table.add_row(vec![Cell::new("Commands").add_attribute(Attribute::Bold)]);
            for (key, data) in self.commands.as_ref().unwrap() {
                table.add_row(vec![
                    "Name\nType\nService\nCharacteristic\nPayload",
                    &format!(
                        "{}\n{}\n{}\n{}\n{}",
                        key,
                        data.command_type,
                        data.service,
                        data.characteristic,
                        data.payload.as_ref().unwrap_or(&"".to_owned())
                    ),
                ]);
            }
        }

        // Functions
        if self.functions.is_some() {
            table.add_row(vec![Cell::new("Functions").add_attribute(Attribute::Bold)]);
            for (key, data) in self.functions.as_ref().unwrap() {
                table.add_row(vec![
                    "Name\nDelays\nCommands",
                    &format!("{}\n{:?}\n{:?}", key, data.commands_delay_ms, data.commands,),
                ]);
            }
        }
        println!("{table}");
    }

    pub async fn run_command(
        &self,
        bt: &mut Box<dyn controllers::BleController>,
        command_name: &str,
    ) -> Result<(), Box<dyn Error>> {
        // check if there are no commands in preset
        if self.commands.is_none() {
            Err("No commands in preset")?;
        }

        // get command struct from typed name
        let command = match self.commands.as_ref().unwrap().get(command_name) {
            Some(s) => s,
            None => Err(format!("Command not found {}", command_name))?,
        };

        let service_def = self
            .services
            .as_ref()
            .unwrap()
            .get(&command.service)
            .unwrap();

        // translate human readable service and characteristics names into their uuids
        let service_uuid = service_def.uuid.clone();
        let characteristic_uuid = service_def
            .characteristics
            .as_ref()
            .unwrap()
            .get(&command.characteristic)
            .unwrap()
            .uuid
            .clone();

        // execute command
        match command.command_type.as_str() {
            "write" => {
                commands::write::write(
                    bt,
                    &service_uuid,
                    &characteristic_uuid,
                    command.payload.as_ref().unwrap(),
                )
                .await?;
            }
            "read" => {
                commands::read::read(bt, &service_uuid, &characteristic_uuid).await?;
            }
            "notify" => (),

            _ => panic!("Invalid command type '{}'", command.command_type),
        };

        Ok(())
    }

    pub async fn run_function(
        &self,
        bt: &mut Box<dyn controllers::BleController>,
        function_name: &str,
    ) -> Result<(), Box<dyn Error>> {
        // check if there are no function in preset
        if self.functions.is_none() {
            Err("No functions in preset")?;
        }

        // get fucntion struct from typed name
        let function = match self.functions.as_ref().unwrap().get(function_name) {
            Some(s) => s,
            None => Err(format!("Command not found {}", function_name))?,
        };

        // run function
        for (index, command_name) in function.commands.iter().enumerate() {
            println!("Running {} ...", command_name);
            self.run_command(bt, command_name).await?;
            println!("Waiting {} ms", function.commands_delay_ms[index]);
            time::sleep(Duration::from_millis(function.commands_delay_ms[index])).await;
        }

        Ok(())
    }

    pub fn should_autoconnect(&self) -> bool {
        if self.device.is_some() {
            return self.device.as_ref().unwrap().autoconnect.unwrap_or(false);
        }
        false
    }

    pub async fn autoconnect(
        &self,
        bt: &mut Box<dyn controllers::BleController>,
    ) -> Result<(), Box<dyn Error>> {
        commands::scan::run(bt, 5, false, false).await.unwrap();
        if self.device.as_ref().unwrap().name.is_some() {
            commands::connect::by_name(bt, &self.device.as_ref().unwrap().name.as_ref().unwrap())
                .await?;
        } else {
            commands::connect::by_address(
                bt,
                &self.device.as_ref().unwrap().address.as_ref().unwrap(),
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
            match Preset::new(path::PathBuf::from(fp)) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("{}", e);
                    assert!(false)
                }
            }
        }
    }
}
