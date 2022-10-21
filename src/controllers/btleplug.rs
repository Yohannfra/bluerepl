use super::{
    BleController, BlePeripheral, BlePeripheralInfo, Characteristic, CharacteristicProperties,
    Service,
};
use async_trait::async_trait;
use std::error::Error;
use std::time::Duration;
use tokio::time;

// mod utils;
use crate::utils;

use btleplug::api::{Central, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::{Adapter, Manager};

pub struct BtleplugController {
    connected: bool,
    adapter: Adapter,
    scan_list: Vec<BlePeripheral>,
}

#[async_trait]
impl BleController for BtleplugController {
    async fn scan(&mut self, scan_time_s: usize) -> Result<(), Box<dyn Error>> {
        println!("Scanning for {} seconds...", scan_time_s);

        self.adapter.start_scan(ScanFilter::default()).await?; // start scan
        time::sleep(Duration::from_secs(scan_time_s as u64)).await;   // wait x seconds
        self.adapter.stop_scan().await?;                              // stop scan

        let peripherals = self.adapter.peripherals().await?;
        let mut periph_vec: Vec<BlePeripheral> = Vec::new();
        let mut index: usize = 0;

        for p in peripherals {
            let properties = p.properties().await?.unwrap();
            let name = properties.local_name.unwrap_or(String::from("unknown"));
            let rssi: i16 = properties.rssi.unwrap_or(0);

            periph_vec.push(BlePeripheral {
                name,
                address_uuid: p.id().to_string(),
                rssi,
                id: index,
            });

            index += 1;
        }
        self.scan_list = periph_vec;
        Ok(())
    }

    fn get_scan_list(&self) -> Vec<BlePeripheral> {
        self.scan_list.clone()
    }

    async fn get_adapter_infos(&self) -> Result<String, Box<dyn Error>> {
        let adapter_infos: String = self.adapter.adapter_info().await?;
        Ok(adapter_infos)
    }

    async fn write(&mut self, _service: &str, characteristic: &str, payload: &[u8]) -> Result<(), Box<dyn Error>> {
        let mut char_found = false;

        for p in &self.adapter.peripherals().await? {
            if p.is_connected().await? {
                for c in p.characteristics() {
                    if c.uuid.to_string() == characteristic {
                        println!("Writing {:?} to characteristic {}", payload, c.uuid.to_string());
                        char_found = true;
                        p.write(&c, payload, btleplug::api::WriteType::WithoutResponse).await?;
                    }
                }
            }
        }

        if char_found == false {
            Err(format!("Characteristic: {} not found", characteristic))?
        }

        Ok(())
    }

    async fn read(&mut self, _service: &str, characteristic: &str) -> Result<(), Box<dyn Error>> {
        let mut char_found = false;

        for p in &self.adapter.peripherals().await? {
            if p.is_connected().await? {
                for c in p.characteristics() {
                    if c.uuid.to_string() == characteristic {
                        char_found = true;
                        println!("Reading Characteristic {} ...", c.uuid.to_string());
                        let content = p.read(&c).await?;
                        println!("{:?}", content);
                    }
                }
            }
        }

        if char_found == false {
            Err(format!("Characteristic: {} not found", characteristic))?
        }

        Ok(())
    }

    async fn get_peripheral_infos(&self) -> Result<BlePeripheralInfo, Box<dyn Error>> {
        for p in &self.adapter.peripherals().await? {
            if p.is_connected().await? {
                let services = p.services();
                let properties = p.properties().await?.unwrap();

                let mut infos = BlePeripheralInfo {
                    periph_name: properties.local_name.unwrap_or(String::from("unknown")),
                    periph_mac: p.id().to_string(),
                    rssi: properties.rssi.unwrap_or(0),
                    services: Vec::new(),
                };

                for s in services {
                    let mut ser = Service {
                        uuid: s.uuid.to_string(),
                        characteriscics: Vec::new(),
                    };

                    for c in s.characteristics {
                        let mut car = Characteristic {
                            uuid: c.uuid.to_string(),
                            properties: CharacteristicProperties::UNKNOWN,
                        };

                        if c.properties.contains(btleplug::api::CharPropFlags::WRITE) {
                            car.properties |= CharacteristicProperties::WRITE;
                        }

                        if c.properties.contains(btleplug::api::CharPropFlags::READ) {
                            car.properties |= CharacteristicProperties::READ;
                        }

                        if c.properties
                            .contains(btleplug::api::CharPropFlags::WRITE_WITHOUT_RESPONSE)
                        {
                            car.properties |= CharacteristicProperties::WRITE_WITHOUT_RESPONSE;
                        }

                        if c.properties.contains(btleplug::api::CharPropFlags::NOTIFY) {
                            car.properties |= CharacteristicProperties::NOTIFY;
                        }

                        if c.properties
                            .contains(btleplug::api::CharPropFlags::INDICATE)
                        {
                            car.properties |= CharacteristicProperties::INDICATE;
                        }

                        ser.characteriscics.push(car);
                    }
                    infos.services.push(ser);
                }
                return Ok(infos);
            }
        }
        panic!("Code should not arrive here");
    }

    async fn connect(&mut self, uuid: &str) -> Result<(), Box<dyn Error>> {
        for p in &self.adapter.peripherals().await? {
            let properties = p.properties().await?.unwrap();
            let name = properties.local_name.unwrap_or(String::from("unknown"));

            if uuid == p.id().to_string() {
                println!("Connecting to {} with uuid: {}", name, p.id().to_string());
                match p.connect().await {
                    Ok(()) => {
                        self.connected = true;
                        return Ok(());
                    }
                    Err(e) => return Err(format!("{}", e))?,
                }
            }
        }
        Err(format!("Peripheral with uuid {} not found", uuid))?
    }

    async fn disconnect(&mut self) -> Result<(), Box<dyn Error>> {
        for p in &self.adapter.peripherals().await? {
            if p.is_connected().await? {
                let properties = p.properties().await?.unwrap();
                let name = properties.local_name.unwrap_or(String::from("unknown"));
                println!(
                    "Disconnecting from {} with uuid: {} ... ",
                    name,
                    p.id().to_string()
                );
                self.connected = false;
                p.disconnect().await?;
                break;
            }
        }
        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.connected
    }
}

impl BtleplugController {
    pub async fn new() -> BtleplugController {
        let manager = match Manager::new().await {
            Ok(m) => m,
            Err(e) => panic!("{:?}", e),
        };

        let adapter_list = match manager.adapters().await {
            Ok(v) => v,
            Err(e) => panic!("{:?}", e),
        };

        let adapter = match adapter_list.len() {
            0 => panic!("No adapter available"),
            1 => &adapter_list[0],
            _ => {
                println!("Found multiple adapters, select the one to use:");
                let mut index = 0;
                for ad in &adapter_list {
                    println!("[{}]: {:?}", index, ad);
                    index += 1;
                }
                let n = utils::get_usize_input(">>");
                &adapter_list[n]
            }
        };

        println!(
            "Using BLE adapter: {:?}",
            adapter.adapter_info().await.unwrap()
        );

        BtleplugController {
            connected: false,
            adapter: adapter.clone(),
            scan_list: Vec::new(),
        }
    }
}
