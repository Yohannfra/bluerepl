use crate::controllers;

use std::error::Error;

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
