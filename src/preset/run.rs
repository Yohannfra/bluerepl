use super::Preset;

use crate::controllers;
use crate::repl::commands;
use std::error::Error;

use std::time::Duration;
use tokio::time;

impl Preset {
    pub async fn run_command(
        &self,
        bt: &mut dyn controllers::BleController,
        command_name: &str,
    ) -> Result<(), Box<dyn Error>> {
        // check if there are no commands in preset
        if self.commands.is_none() {
            Err("No commands in preset")?;
        }

        // get command struct from typed name
        let Some(command) = self.commands.as_ref().unwrap().get(command_name) else {
            return Err(format!("Command not found {}", command_name))?;
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
                    false,
                )
                .await?;
            }
            "write_with_resp" => {
                commands::write::write(
                    bt,
                    &service_uuid,
                    &characteristic_uuid,
                    command.payload.as_ref().unwrap(),
                    true,
                )
                .await?;
            }
            "read" => {
                commands::read::read(bt, &service_uuid, &characteristic_uuid, &command.format)
                    .await?;
            }
            "notify" => {
                commands::notify::notify(bt, &service_uuid, &characteristic_uuid).await?;
            }
            "indicate" => {
                commands::indicate::indicate(bt, &service_uuid, &characteristic_uuid).await?;
            }
            "unsubscribe" => {
                commands::unsubscribe::unsubscribe(bt, &service_uuid, &characteristic_uuid).await?;
            }

            _ => panic!("Invalid command type '{}'", command.command_type),
        };

        Ok(())
    }

    pub async fn run_function(
        &self,
        bt: &mut dyn controllers::BleController,
        function_name: &str,
    ) -> Result<(), Box<dyn Error>> {
        // check if there are no function in preset
        if self.functions.is_none() {
            Err("No functions in preset")?;
        }

        // get function struct from typed name
        let Some(function) = self.functions.as_ref().unwrap().get(function_name) else {
            return Err(format!("Command not found {}", function_name))?;
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
}
