use super::compare_uuid::compare_uuid;
use serde::{Deserialize, Serialize};

use crate::Preset;

static CHARACTERISTICS_UUIDS_JSON_STR: &str = include_str!("characteristic_uuids.json");

#[derive(Serialize, Deserialize, Debug)]
struct Characteristic {
    name: String,
    identifier: String,
    uuid: String,
    source: String,
}

lazy_static! {
    static ref PARSED_JSON: Vec<Characteristic> =
        serde_json::from_str(CHARACTERISTICS_UUIDS_JSON_STR).unwrap();
}

pub fn get_characteristic_name_from_uuid(
    uuid_ser: &str,
    uuid_char: &str,
    p: &Option<Preset>,
) -> Option<String> {
    for s in PARSED_JSON.iter() {
        if compare_uuid(&uuid_ser.to_uppercase(), &s.uuid.to_uppercase()) {
            return Some(s.name.clone());
        }
    }

    if let Some(preset) = p {
        if let Some(name) = preset.get_characteristic_name_from_uuid(uuid_ser, uuid_char) {
            return Some(name);
        }
    }

    None
}

pub fn get_characteristic_identifier_from_uuid(uuid: &str) -> Option<String> {
    for s in PARSED_JSON.iter() {
        if compare_uuid(&uuid.to_uppercase(), &s.uuid.to_uppercase()) {
            return Some(s.identifier.clone());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_characteristic_name_from_uuid() {
        assert_eq!(
            get_characteristic_name_from_uuid("2A19"),
            Some("Battery Level".to_owned())
        );
        assert_eq!(
            get_characteristic_name_from_uuid("2A8D"),
            Some("Heart Rate Max".to_owned())
        );
        assert_eq!(
            get_characteristic_name_from_uuid("ADAF0E01-C332-42A8-93BD-25E905756CB8"),
            Some("Adafruit Proximity".to_owned())
        );
    }

    #[test]
    fn test_get_characteristic_identifier_from_uuid() {
        assert_eq!(
            get_characteristic_identifier_from_uuid("2A19"),
            Some("org.bluetooth.characteristic.battery_level".to_owned())
        );
        assert_eq!(
            get_characteristic_identifier_from_uuid("2A8D"),
            Some("org.bluetooth.characteristic.heart_rate_max".to_owned())
        );
        assert_eq!(
            get_characteristic_identifier_from_uuid("ADAF0E01-C332-42A8-93BD-25E905756CB8"),
            Some("com.adafruit.characteristic.proximity".to_owned())
        );
    }
}
