use crate::controllers;
use std::error::Error;

pub async fn read(
    bt: &mut dyn controllers::BleController,
    service: &str,
    characteristic: &str,
) -> Result<(), Box<dyn Error>> {
    if !bt.is_connected() {
        Err("You must be connected to a peripheral to run this command")?;
    }

    bt.read(service, characteristic).await
}
