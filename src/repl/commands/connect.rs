use crate::controllers;

use controllers::BlePeripheral;
use std::error::Error;

pub async fn by_name(
    bt: &mut Box<dyn controllers::BleController>,
    list: &Vec<BlePeripheral>,
    name: &str,
) -> Result<(), Box<dyn Error>> {
    match list.iter().find(|e| e.name == name) {
        Some(p) => {
            println!("Connecting with name: {}", name);
            bt.connect(&p.address_uuid).await
        }
        None => {
            println!("Name not found");
            return Ok(());
        }
    }
}

pub async fn by_index(
    bt: &mut Box<dyn controllers::BleController>,
    list: &Vec<BlePeripheral>,
    id: usize,
) -> Result<(), Box<dyn Error>> {
    println!("Connecting with id: {}", id);

    match list.iter().find(|e| e.id == id) {
        Some(p) => {
            println!("Connecting with id: {}", id);
            bt.connect(&p.address_uuid).await
        }
        None => {
            println!("Id not found");
            return Ok(());
        }
    }
}

pub async fn by_address(
    bt: &mut Box<dyn controllers::BleController>,
    list: &Vec<BlePeripheral>,
    addr: &str,
) -> Result<(), Box<dyn Error>> {
    match list.iter().find(|e| e.address_uuid == addr) {
        Some(p) => {
            println!("Connecting with address: {}", addr);
            bt.connect(&p.address_uuid).await
        }
        None => {
            println!("Name not found");
            return Ok(());
        }
    }
}
