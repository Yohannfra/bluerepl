use super::{Command, Function, Preset};

impl Preset {
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

        // check that if autoconnect=true there is also device name or address
        if let Some(device) = &self.device {
            if device.autoconnect.unwrap_or(false)
                && device.name.is_none()
                && device.address.is_none()
            {
                panic!("You must provide a name or an address to use the autoconnect feature");
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
    }
}
