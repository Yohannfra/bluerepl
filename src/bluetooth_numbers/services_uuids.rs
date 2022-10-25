use serde::{Deserialize, Serialize};

static SERVICES_UUIDS_JSON_STR: &'static str = include_str!("service_uuids.json");

#[derive(Serialize, Deserialize, Debug)]
struct Services {
    name: String,
    identifier: String,
    uuid: String,
    source: String,
}

lazy_static! {
    static ref PARSED_JSON: Vec<Services> = serde_json::from_str(SERVICES_UUIDS_JSON_STR).unwrap();
}

pub fn get_service_name_from_uuid(uuid: &str) -> Option<String> {
    for s in PARSED_JSON.iter() {
        if s.uuid == uuid {
            return Some(s.name.clone());
        }
    }
    return None;
}

pub fn get_service_identifier_from_uuid(uuid: &str) -> Option<String> {
    for s in PARSED_JSON.iter() {
        if s.uuid == uuid {
            return Some(s.identifier.clone());
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_service_name_from_uuid() {
        assert_eq!(get_service_name_from_uuid("180F"), Some("Battery Service".to_owned()));
        assert_eq!(get_service_name_from_uuid("180D"), Some("Heart Rate".to_owned()));
        assert_eq!(get_service_name_from_uuid("ADAF0E00-C332-42A8-93BD-25E905756CB8"), Some("Adafruit Proximity Service".to_owned()));

    }

    #[test]
    fn test_get_service_identifier_from_uuid() {
        assert_eq!(get_service_identifier_from_uuid("180F"), Some("org.bluetooth.service.battery_service".to_owned()));
        assert_eq!(get_service_identifier_from_uuid("180D"), Some("org.bluetooth.service.heart_rate".to_owned()));
        assert_eq!(get_service_identifier_from_uuid("ADAF0E00-C332-42A8-93BD-25E905756CB8"), Some("com.adafruit.service.proximity".to_owned()));

    }
}
