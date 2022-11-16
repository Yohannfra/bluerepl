use crate::controllers;
use std::error::Error;

use crate::utils::print_bytes;

pub async fn read(
    bt: &mut dyn controllers::BleController,
    service: &str,
    characteristic: &str,
    format: &str,
) -> Result<(), Box<dyn Error>> {
    if !bt.is_connected() {
        Err("You must be connected to a peripheral to run this command")?;
    }

    let bytes_read = bt.read(service, characteristic).await?;
    println!("{}", print_bytes::bytes_to_str(&bytes_read, format));

    Ok(())
}

pub async fn read_as_str(
    bt: &mut dyn controllers::BleController,
    service: &str,
    characteristic: &str,
    format: &str,
) -> Result<String, Box<dyn Error>> {
    if !bt.is_connected() {
        Err("You must be connected to a peripheral to run this command")?;
    }

    let bytes_read = bt.read(service, characteristic).await?;

    Ok(print_bytes::bytes_to_str(&bytes_read, format))
}
