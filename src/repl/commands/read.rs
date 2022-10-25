use crate::controllers;
use std::error::Error;

pub async fn read(
    bt: &mut Box<dyn controllers::BleController>,
    service: &str,
    characteristic: &str,
) -> Result<(), Box<dyn Error>> {
    if bt.is_connected() == false {
        Err("You must be connected to a peripheral to run this command")?;
    }

    bt.read(&service, &characteristic).await
}
