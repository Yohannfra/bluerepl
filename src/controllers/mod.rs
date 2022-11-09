use async_trait::async_trait;
use std::error::Error;

use bitflags::bitflags;

pub mod btleplug;
pub mod simpleble;

#[derive(Debug, Clone)]
pub struct BlePeripheral {
    /// Internal id used by ble implementations
    pub id: usize,

    /// peripheral name
    pub name: String,

    /// Peripheral mac address (or Apple generated UUID on OSX)
    pub address_uuid: String,

    /// peripheral rssi
    pub rssi: i16,

    /// company_id
    pub company_id: usize,
}

bitflags! {
    pub struct CharacteristicProperties: u8 {
        const UNKNOWN                = 0b00000000;
        const READ                   = 0b00000001;
        const WRITE                  = 0b00000010;
        const WRITE_WITHOUT_RESPONSE = 0b00000100;
        const NOTIFY                 = 0b00001000;
        const INDICATE               = 0b00010000;
    }
}

#[derive(Debug)]
pub struct Characteristic {
    /// Characteristic UUID
    pub uuid: String,

    /// Characteristic properties (READ, WRITE ...)
    pub properties: CharacteristicProperties,
}

#[derive(Debug)]
pub struct Service {
    /// Service UUID
    pub uuid: String,

    /// Service characteristics
    pub characteriscics: Vec<Characteristic>,
}

#[derive(Debug)]
pub struct BlePeripheralInfo {
    /// Peripheral advertising name
    pub periph_name: String,

    /// Peripheral mac address (or Apple generated UUID on OSX)
    pub periph_mac: String,

    /// Peripheral list of services
    pub services: Vec<Service>,

    /// Peripheral rssi
    pub rssi: i16,
}

#[async_trait]
pub trait BleController {
    async fn scan(&mut self, scan_time_s: usize) -> Result<(), Box<dyn Error>>;

    fn get_scan_list(&self) -> Vec<BlePeripheral>;

    async fn get_adapter_infos(&self) -> Result<String, Box<dyn Error>>;

    async fn get_peripheral_infos(&self) -> Result<BlePeripheralInfo, Box<dyn Error>>;

    async fn connect(&mut self, uuid: &str) -> Result<(), Box<dyn Error>>;

    async fn disconnect(&mut self) -> Result<(), Box<dyn Error>>;

    async fn write(
        &mut self,
        service: &str,
        characteristic: &str,
        payload: &[u8],
        response: bool,
    ) -> Result<(), Box<dyn Error>>;

    async fn read(
        &mut self,
        service: &str,
        characteristic: &str,
    ) -> Result<Vec<u8>, Box<dyn Error>>;

    async fn notify(&mut self, service: &str, characteristic: &str) -> Result<(), Box<dyn Error>>;

    async fn indicate(&mut self, service: &str, characteristic: &str)
        -> Result<(), Box<dyn Error>>;

    async fn unsubscribe(
        &mut self,
        service: &str,
        characteristic: &str,
    ) -> Result<(), Box<dyn Error>>;

    fn is_connected(&self) -> bool;
}
