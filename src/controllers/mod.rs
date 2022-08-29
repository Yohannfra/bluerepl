use async_trait::async_trait;
use std::error::Error;

pub mod btleplug;
pub mod simpleble;

#[derive(Debug)]
pub struct BlePeripheral {
    name: String,
    mac_addr: String,
}

#[async_trait]
pub trait BleController {
    async fn scan(
        &self,
        scan_time_s: u32,
        print_result: bool,
    ) -> Result<Vec<BlePeripheral>, Box<dyn Error>>;

    fn connect_by_name(&self, name: String) -> Result<(), Box<dyn Error>>;
    fn connect_by_mac(&self, mac: String) -> Result<(), Box<dyn Error>>;

    fn disconnect(&self) -> Result<(), Box<dyn Error>>;

    fn is_connected(&self) -> bool;
}
