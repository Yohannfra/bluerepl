use crate::controllers;
use std::error::Error;

pub async fn run(bt: &mut Box<dyn controllers::BleController>)  -> Result<(), Box<dyn Error>>{
    bt.disconnect().await
}
