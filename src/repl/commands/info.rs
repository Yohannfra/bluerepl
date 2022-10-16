use crate::controllers;

use std::error::Error;

pub async fn gatt(bt: &mut Box<dyn controllers::BleController>) -> Result<(), Box<dyn Error>> {
    Ok(())
}
