use super::Preset;

impl Preset {
    pub fn get_service_name_from_uuid(&self, uuid: &str) -> Option<String> {
        let Some(services) = &self.services else {
            return None;
        };

        for (ser_name, ser_data) in services.iter() {
            if ser_data.uuid == uuid {
                return Some(ser_name.to_owned());
            }
        }
        None
    }

    pub fn get_service_uuid_from_name(&self, name: &str) -> Option<String> {
        let Some(services) = &self.services else {
            return None;
        };

        if let Some(service) = services.get(name) {
            return Some(service.uuid.to_owned());
        }

        None
    }

    pub fn get_characteristic_name_from_uuid(
        &self,
        ser_uuid: &str,
        char_uuid: &str,
    ) -> Option<String> {
        let Some(services) = &self.services else {
            return None;
        };

        for (_, ser_data) in services.iter() {
            if ser_data.uuid == ser_uuid {
                let Some(characteristics) = &ser_data.characteristics else {
                    return None;
                };

                for (char_name, char_data) in characteristics.iter() {
                    if char_data.uuid == char_uuid {
                        return Some(char_name.to_owned());
                    }
                }
            }
        }
        None
    }

    pub fn get_characteristic_uuid_from_name(
        &self,
        service_name: &str,
        characteristic_name: &str,
    ) -> Option<String> {
        let Some(services) = &self.services else {
            return None;
        };

        if let Some(service) = services.get(service_name) {
            let Some(characteristics) = &service.characteristics else {
                return None;
            };

            if let Some(characteristic) = characteristics.get(characteristic_name) {
                return Some(characteristic.uuid.to_owned());
            }
        } else {
            // Quick hack in case the service_name is not a name but the service uuid,
            // this fixes potential cases and simplify code everywhere this functions is called
            if let Some(ser_name) = self.get_service_name_from_uuid(service_name) {
                return self.get_characteristic_uuid_from_name(&ser_name, characteristic_name);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path;

    #[test]
    fn test_getters() {
        let pr = Preset::new(path::PathBuf::from("presets/neopixel_controller.toml"));
        assert!(pr.is_ok(), "{:?}", pr);
        let pr = pr.unwrap();

        assert_eq!(
            pr.get_service_name_from_uuid("8e72bbe5-f777-5284-7849-b4a0b2ac70d2"),
            Some("neopixels".to_owned())
        );
        assert_eq!(
            pr.get_service_name_from_uuid("8e72bbe5-f777-5284-7849-b4a0b2ac70d"),
            None
        );

        assert_eq!(
            pr.get_service_uuid_from_name("neopixels"),
            Some("8e72bbe5-f777-5284-7849-b4a0b2ac70d2".to_owned())
        );
        assert_eq!(pr.get_service_uuid_from_name("neopixel"), None);

        assert_eq!(
            pr.get_characteristic_name_from_uuid(
                "8e72bbe5-f777-5284-7849-b4a0b2ac70d2",
                "0000beb6-0000-1000-8000-00805f9b34fb"
            ),
            Some("write".to_owned())
        );
        assert_eq!(
            pr.get_characteristic_name_from_uuid(
                "8e72bbe5-f777-5284-7849-b4a0b2ac70d",
                "0000beb6-0000-1000-8000-00805f9b34fb"
            ),
            None
        );
        assert_eq!(
            pr.get_characteristic_name_from_uuid(
                "8e72bbe5-f777-5284-7849-b4a0b2ac70d2",
                "0000beb6-0000-1000-8000-00805f9b34f"
            ),
            None
        );

        assert_eq!(
            pr.get_characteristic_uuid_from_name("neopixels", "write"),
            Some("0000beb6-0000-1000-8000-00805f9b34fb".to_owned())
        );
        assert_eq!(
            pr.get_characteristic_uuid_from_name("neopixel", "write"),
            None
        );
        assert_eq!(
            pr.get_characteristic_uuid_from_name("neopixels", "writ"),
            None
        );
    }
}
