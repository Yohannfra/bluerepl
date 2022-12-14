use super::Preset;

impl Preset {
    pub fn is_autoconnect_possible(&self) -> bool {
        let Some(device) = &self.device else {
            return false;
        };
        if device.name.is_none() && device.address.is_none() {
            return false;
        }
        true
    }

    pub fn verify(&self) {
        // check if services and characteristics typed in commands descriptions are defined in
        // the preset
        if let Some(commands) = &self.commands {
            // check services
            for cmd in commands {
                if !self.services.as_ref().unwrap().contains_key(&cmd.1.service) {
                    panic!(
                        "Service '{}' in command '{}' not found",
                        cmd.1.service, cmd.0
                    );
                }

                // check characteristics
                for ser in self.services.as_ref().unwrap() {
                    if cmd.1.service == *ser.0
                        && !ser
                            .1
                            .characteristics
                            .as_ref()
                            .unwrap()
                            .contains_key(&cmd.1.characteristic)
                    {
                        panic!(
                            "Characteristic '{}' in command '{}' not found",
                            cmd.1.characteristic, cmd.0
                        );
                    }
                }
            }
        }

        // check that the function commands array is the same length as the function delay array
        if let Some(functions) = &self.functions {
            for f in functions {
                if f.1.commands.len() != f.1.commands_delay_ms.len() {
                    panic!("In function {} 'commands' and 'commands_delay_ms' don't have the same length", f.0)
                }
            }
        }

        // check that commands used in functions exist
        if let Some(functions) = &self.functions {
            for f in functions {
                for cmd_name in &f.1.commands {
                    if !self
                        .commands
                        .as_ref()
                        .unwrap()
                        .iter()
                        .any(|c| c.0 == cmd_name)
                    {
                        panic!("In function '{}' command '{}' doesn't exits", f.0, cmd_name);
                    }
                }
            }
        }

        // check that commands of type read have a payload

        if let Some(commands) = &self.commands {
            for (cmd_name, cmd_data) in commands {
                if cmd_data.command_type == "write" && cmd_data.payload.is_none() {
                    panic!("In command '{}' missing payload", cmd_name);
                }
            }
        }

        // check that the command_type field in commands exists
        if let Some(commands) = &self.commands {
            let available_commands_types: Vec<&str> = vec![
                "write",
                "write_with_resp",
                "read",
                "notify",
                "indicate",
                "unsubscribe",
            ];

            for (cmd_name, cmd_data) in commands {
                if !available_commands_types
                    .iter()
                    .any(|c| *c == cmd_data.command_type)
                {
                    panic!(
                        "In command '{}' invalid command_type: '{}'. It must be one of {:?}",
                        cmd_name, cmd_data.command_type, available_commands_types
                    );
                }
            }
        }

        // check that the format field in commands exists
        if let Some(commands) = &self.commands {
            let available_formats: Vec<&str> = vec!["bin", "dec", "hex", "text", "hexdump"];

            for (cmd_name, cmd_data) in commands {
                if !available_formats.iter().any(|c| *c == cmd_data.format) {
                    panic!(
                        "In command '{}' invalid format: '{}'. It must be one of {:?}",
                        cmd_name, cmd_data.format, available_formats
                    );
                }
            }
        }
    }
}
