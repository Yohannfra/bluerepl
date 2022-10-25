use serde::{Deserialize, Serialize};

static DESCRIPTOR_UUIDS_JSON_STR: &'static str = include_str!("descriptor_uuids.json");

#[derive(Serialize, Deserialize, Debug)]
struct Services {
    name: String,
    identifier: String,
    uuid: String,
    source: String,
}

lazy_static! {
    static ref PARSED_JSON: Vec<Services> = serde_json::from_str(DESCRIPTOR_UUIDS_JSON_STR).unwrap();
}

pub fn get_descriptor_name_from_uuid(uuid: &str) -> Option<String> {
    for s in PARSED_JSON.iter() {
        if s.uuid.to_uppercase() == uuid.to_uppercase() {
            return Some(s.name.clone());
        }
    }
    return None;
}

pub fn get_descriptor_identifier_from_uuid(uuid: &str) -> Option<String> {
    for s in PARSED_JSON.iter() {
        if s.uuid.to_uppercase() == uuid.to_uppercase() {
            return Some(s.identifier.clone());
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_descriptor_name_from_uuid() {
        assert_eq!(get_descriptor_name_from_uuid("2906"), Some("Valid Range".to_owned()));
        assert_eq!(get_descriptor_name_from_uuid("290F"), Some("Complete BR-EDR Transport Block Data".to_owned()));

    }

    #[test]
    fn test_get_descriptor_identifier_from_uuid() {
        assert_eq!(get_descriptor_identifier_from_uuid("2906"), Some("org.bluetooth.descriptor.valid_range".to_owned()));
        assert_eq!(get_descriptor_identifier_from_uuid("290F"), Some("org.bluetooth.descriptor.complete_bredr_transport_block_data".to_owned()));

    }
}
