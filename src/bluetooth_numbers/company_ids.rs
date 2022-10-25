use serde::{Deserialize, Serialize};

static COMPANY_ID_JSON_STR: &'static str = include_str!("company_ids.json");

#[derive(Serialize, Deserialize, Debug)]
struct Company {
    code: usize,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CompanyList {
    list: Vec<Company>
}

lazy_static! {
    static ref PARSED_JSON: CompanyList = serde_json::from_str(COMPANY_ID_JSON_STR).unwrap();
}

pub fn get_company_name_from_id(code: usize) -> Option<String> {
    for k in &PARSED_JSON.list {
        if k.code == code {
            return Some(k.name.clone());
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_company_name_from_id() {
        assert_eq!(get_company_name_from_id(0), Some("Ericsson Technology Licensing".to_owned()));
        assert_eq!(get_company_name_from_id(89), Some("Nordic Semiconductor ASA".to_owned()));
        assert_eq!(get_company_name_from_id(2087), Some("Kickmaker".to_owned()));
        assert_eq!(get_company_name_from_id(65535), Some("Bluetooth SIG Specification Reserved Default Vendor ID for Remote Devices Without Device ID Service Record.".to_owned()));
        assert_eq!(get_company_name_from_id(4124), None);
        assert_eq!(get_company_name_from_id(6313), None);
    }
}
