use super::{BleController, BlePeripheral};
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
    async fn scan(&self, scan_time_s: usize) -> Result<Vec<BlePeripheral>, Box<dyn Error>> {
        println!("Scanning for {} seconds...", scan_time_s);

        self.adapter
            .start_scan(ScanFilter::default())
            .await
            .expect("Can't scan BLE adapter for connected devices...");
        time::sleep(Duration::from_secs(scan_time_s as u64)).await;

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
        Ok(periph_vec)
    }

    fn get_scan_list(&self) -> Vec<BlePeripheral> {
        self.scan_list.clone()
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
                        println!("{:?}", p.services());
                        println!("{:?}", p.characteristics());
                        return Ok(());
                    }
                    Err(e) => return Err(format!("{}", e).as_str().into()),
                }
            }
        }
        Err(format!("Peripheral with uuid {} not found", uuid)
            .as_str()
            .into())
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
                println!("Disconnect");
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
