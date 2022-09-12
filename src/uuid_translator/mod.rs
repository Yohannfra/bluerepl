use serde_json::{Result, Value};

/*
 The file ble_16_bits_uuids.json was generated with this script:
https://github.com/Yohannfra/generate_bluetooth_16_bits_uuid_json
 */

pub fn translate_uuid(uuid_to_find: &str) -> Option<String> {
    let json_str = include_str!("ble_16_bits_uuids.json");

    let json_data: Value = serde_json::from_str(json_str).unwrap();

    let top_keys: [&str; 9] = [
        "16-bit UUID for Members",
        "GATT Characteristic and Object Type",
        "GATT Declarations ",
        "GATT Descriptor",
        "GATT Service",
        "GATT Unit ",
        "Protocol Identifier",
        "SDO GATT Service",
        "Service Classes and Profiles",
    ];

    for key in top_keys {
        for item in json_data[key].as_array().unwrap() {
            for (uuid, name) in item.as_object().unwrap().iter() {
                if uuid.to_lowercase() == uuid_to_find.to_lowercase() {
                    return Some(name.to_string());
                }
            }
        }
    }
    None
}
