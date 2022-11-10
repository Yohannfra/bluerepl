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
}
