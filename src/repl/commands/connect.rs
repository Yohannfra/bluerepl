use crate::controllers;

use std::error::Error;
use std::env;
use regex::Regex;

pub async fn by_name(
    bt: &mut Box<dyn controllers::BleController>,
    name: &str,
) -> Result<(), Box<dyn Error>> {
    match bt.get_scan_list().iter().find(|e| e.name == name) {
        Some(p) => {
            println!("Connecting with name: {}", name);
            bt.connect(&p.address_uuid).await?;
        }
        None => Err("Name not found")?,
    }
    Ok(())
}

pub async fn by_index(
    bt: &mut Box<dyn controllers::BleController>,
    id: usize,
) -> Result<(), Box<dyn Error>> {
    match bt.get_scan_list().iter().find(|e| e.id == id) {
        Some(p) => {
            println!("Connecting with id: {}", id);
            bt.connect(&p.address_uuid).await?;
        }
        None => Err("Id not found")?,
    }
    Ok(())
}

pub async fn by_address(
    bt: &mut Box<dyn controllers::BleController>,
    addr: &str,
) -> Result<(), Box<dyn Error>> {
    match bt.get_scan_list().iter().find(|e| e.address_uuid == addr) {
        Some(p) => {
            println!("Connecting with address: {}", addr);
            bt.connect(&p.address_uuid).await?;
        }
        None => Err("Address not found")?,
    }
    Ok(())
}

pub async fn auto_detect_identifier(
    bt: &mut Box<dyn controllers::BleController>,
    identifier: &str,
    ) -> Result<(), Box<dyn Error>> {

    // try index
    match identifier.parse::<usize>() {
        Ok(n) => return by_index(bt, n).await,
        Err(_) => (),
    };

    // try mac address (or id on OSX)
    if env::consts::OS == "macos" {
        let re = Regex::new(r"^^[a-z0-9]{8}-[a-z0-9]{4}-[a-z0-9]{4}-[a-z0-9]{4}-[a-z0-9]{12}$").unwrap();
        if re.is_match(identifier) {
            return by_address(bt, identifier).await;
        }
    } else {
        let re = Regex::new(r"^([0-9A-Fa-f]{2}[:-]){5}[0-9A-Fa-f]{2}$").unwrap();
        if re.is_match(identifier) {
            return by_address(bt, identifier).await;
        }
    }

    // try name
    return by_name(bt, identifier).await;
}
