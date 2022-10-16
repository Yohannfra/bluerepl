use async_trait::async_trait;
use std::error::Error;

pub mod btleplug;
pub mod simpleble;

#[derive(Debug, Clone)]
pub struct BlePeripheral {
    /// Internal id used by ble implementations
    pub id: usize,

    /// peripheral name
    pub name: String,

    /// ble 128bytes uuid address
    pub address_uuid: String,

    /// peripheral rssi
    pub rssi: i16,
}

#[async_trait]
pub trait BleController {
    async fn scan(&mut self, scan_time_s: usize) -> Result<(), Box<dyn Error>>;

    fn get_scan_list(&self) -> Vec<BlePeripheral>;

    async fn connect(&mut self, uuid: &str) -> Result<(), Box<dyn Error>>;

    async fn disconnect(&mut self) -> Result<(), Box<dyn Error>>;

    fn is_connected(&self) -> bool;
}
