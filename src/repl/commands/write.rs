use crate::controllers;
use std::error::Error;

use str_to_bytes::str_to_bytes;

pub async fn write(
    bt: &mut Box<dyn controllers::BleController>,
    service: &str,
    characteristic: &str,
    payload: &str,
) -> Result<(), Box<dyn Error>> {

    if bt.is_connected() == false {
        Err("You must be connected to a peripheral to run this command")?;
    }

    let pl: Vec<u8> = str_to_bytes(payload)?;

    bt.write(&service, &characteristic, &pl).await
}
