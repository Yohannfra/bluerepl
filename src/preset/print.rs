use super::Preset;
use comfy_table::{Attribute, Cell, Table};

impl Preset {
    pub fn print(&self) {
        let mut table = Table::new();

        table.set_header(vec![
            Cell::new("File name:").add_attribute(Attribute::Bold),
            Cell::new(self.fp.to_string_lossy()),
        ]);

        // Device infos
        if let Some(device) = &self.device {
            table.add_row(vec![
                "Device name\nDevice address",
                &format!(
                    "{}\n{}",
                    device.name.as_ref().unwrap_or(&"".to_owned()),
                    device.address.as_ref().unwrap_or(&"".to_owned()),
                ),
            ]);
        }

        // Service(s)
        if let Some(services) = &self.services {
            table.add_row(vec![Cell::new("Service").add_attribute(Attribute::Bold)]);

            for (key, ser) in services {
                let fmt_service = format!("{}\n{}", key, ser.uuid);
                let mut vec_service = vec!["Name\nUUID".to_owned(), fmt_service];

                if let Some(service_characteristics) = &ser.characteristics {
                    for (key, charac) in service_characteristics {
                        vec_service[0].push_str("\n\nCharacteristic:\n");
                        vec_service[0].push_str(" - Name:\n - UUID");
                        vec_service[1].push_str(&format!("\n\n\n{}\n{}", key, charac.uuid));
                    }
                }
                table.add_row(vec_service);
            }
        }

        // Commands
        if let Some(commands) = &self.commands {
            table.add_row(vec![Cell::new("Commands").add_attribute(Attribute::Bold)]);
            for (key, data) in commands {
                let mut col1: String = "Name\nType\nService\nCharacteristic".to_owned();
                let mut col2: String = format!(
                    "{}\n{}\n{}\n{}",
                    key, data.command_type, data.service, data.characteristic
                );

                if data.command_type == "write" {
                    col1.push_str("\nPayload");
                    col2.push_str(&format!("\n{}", data.payload.as_ref().unwrap()));
                }

                if data.command_type == "read"
                    || data.command_type == "indicate"
                    || data.command_type == "notify"
                {
                    col1.push_str("\nFormat");
                    col2.push_str(&format!("\n{}", data.format));
                }

                table.add_row(vec![col1, col2]);
            }
        }

        // Functions
        if let Some(functions) = &self.functions {
            table.add_row(vec![Cell::new("Functions").add_attribute(Attribute::Bold)]);
            for (key, data) in functions {
                table.add_row(vec![
                    "Name\nDelays\nCommands",
                    &format!("{}\n{:?}\n{:?}", key, data.commands_delay_ms, data.commands),
                ]);
            }
        }
        println!("{table}");
    }

    pub fn print_commands(&self) {
        // TODO change data orientation

        if let Some(commands) = &self.commands {
            let mut table = Table::new();

            table.add_row(vec![Cell::new("Commands").add_attribute(Attribute::Bold)]);
            for (key, data) in commands {
                let mut col1: String = "Name\nType\nService\nCharacteristic".to_owned();
                let mut col2: String = format!(
                    "{}\n{}\n{}\n{}",
                    key, data.command_type, data.service, data.characteristic
                );

                if data.command_type == "write" {
                    col1.push_str("\nPayload");
                    col2.push_str(&format!("\n{}", data.payload.as_ref().unwrap()));
                }

                if data.command_type == "read"
                    || data.command_type == "indicate"
                    || data.command_type == "notify"
                {
                    col1.push_str("\nFormat");
                    col2.push_str(&format!("\n{}", data.format));
                }

                table.add_row(vec![col1, col2]);
            }
            println!("{table}");
        }
    }
}
