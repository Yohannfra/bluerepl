use crate::controllers;
use std::error::Error;

pub async fn unsubscribe(
    bt: &mut dyn controllers::BleController,
    service: &str,
    characteristic: &str,
) -> Result<(), Box<dyn Error>> {
    if !bt.is_connected() {
        Err("You must be connected to a peripheral to run this command")?;
    }

    bt.unsubscribe(service, characteristic).await
}
