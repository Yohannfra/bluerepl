pub fn compare_uuid(uuid_to_check: &str, uuid_ref: &str) -> bool {
    if uuid_to_check == uuid_ref {
        return true;
    }

    if uuid_to_check == "0000".to_owned() + uuid_ref + "-0000-1000-8000-00805F9B34FB" {
        return true;
    }

    false
}
