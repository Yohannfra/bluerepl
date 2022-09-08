use async_trait::async_trait;
use std::error::Error;

use super::{BleController, BlePeripheral};

pub struct SimpleBleController {
    controller_name: String,
    peripheral_name: Option<String>,
    peripheral_mac: Option<String>,
    connected: bool,
}

#[async_trait]
impl BleController for SimpleBleController {
    async fn scan(&self, scan_time_s: u32) -> Result<Vec<BlePeripheral>, Box<dyn Error>> {
        println!("Scanning for {} seconds...", scan_time_s);
        Ok(Vec::new())
    }

    fn connect_by_name(&self, name: String) -> Result<(), Box<dyn Error>> {
        println!("Connecting to device named {}", name);
        Ok(())
    }

    fn connect_by_mac(&self, mac: String) -> Result<(), Box<dyn Error>> {
        println!("Connecting to device with mac address {}", mac);
        Ok(())
    }

    fn disconnect(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.connected
    }
}

impl SimpleBleController {
    pub fn new() -> SimpleBleController {
        SimpleBleController {
            controller_name: String::from("simpleble"),
            peripheral_name: None,
            peripheral_mac: None,
            connected: false,
        }
    }
}
