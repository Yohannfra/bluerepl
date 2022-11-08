use super::Preset;
use comfy_table::{Attribute, Cell, Table};

impl Preset {
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
}
