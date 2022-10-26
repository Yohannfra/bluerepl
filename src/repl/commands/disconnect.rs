use crate::controllers;
use std::error::Error;

pub async fn run(bt: &mut dyn controllers::BleController) -> Result<(), Box<dyn Error>> {
    bt.disconnect().await?;

    println!("Disconnected");

    Ok(())
}
