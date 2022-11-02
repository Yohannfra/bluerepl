use super::{
    BleController, BlePeripheral, BlePeripheralInfo, Characteristic, CharacteristicProperties,
    Service,
};

use async_trait::async_trait;
use futures::executor::block_on;
use futures::stream::StreamExt;
use std::error::Error;
use std::thread;
use std::time::Duration;
use tokio::time;

use crate::utils;

use btleplug::api::{Central, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::{Adapter, Manager};

use std::sync::atomic;
use std::sync::Arc;

pub struct BtleplugController {
    adapter: Adapter,
    scan_list: Vec<BlePeripheral>,
    peripheral: Option<Box<btleplug::platform::Peripheral>>,
    notifications_thread_running: Arc<atomic::AtomicBool>,
}

#[async_trait]
impl BleController for BtleplugController {
    async fn scan(&mut self, scan_time_s: usize) -> Result<(), Box<dyn Error>> {
        println!("Scanning for {} seconds...", scan_time_s);

        // stop previous scan, return values doesn't matter
        let _ = self.adapter.stop_scan().await;

        self.adapter.start_scan(ScanFilter::default()).await?; // start scan
        time::sleep(Duration::from_secs(scan_time_s as u64)).await; // wait x seconds

        let peripherals = self.adapter.peripherals().await?;
        let mut periph_vec: Vec<BlePeripheral> = Vec::new();

        for (index, p) in peripherals.into_iter().enumerate() {
            let properties = p.properties().await?.unwrap();
            let name = properties
                .local_name
                .unwrap_or_else(|| String::from("unknown"));
            let mut company_code = std::usize::MAX;
            if let Some((code, _)) = properties.manufacturer_data.iter().next() {
                company_code = *code as usize;
            }

            let rssi: i16 = properties.rssi.unwrap_or_else(|| 0);

            periph_vec.push(BlePeripheral {
                name,
                address_uuid: self.get_address_or_uuid(&p).await?,
                rssi,
                id: index,
                company_id: company_code,
            });
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

    async fn write(
        &mut self,
        _service: &str,
        characteristic: &str,
        payload: &[u8],
    ) -> Result<(), Box<dyn Error>> {
        if let Some(p) = &self.peripheral {
            p.discover_services().await.unwrap();

            let c = p
                .characteristics()
                .into_iter()
                .find(|c| c.uuid.to_string() == characteristic);

            if let Some(c) = c {
                println!("Writing {:?} to characteristic {}", payload, c.uuid);
                p.write(&c, payload, btleplug::api::WriteType::WithoutResponse)
                    .await?;
            } else {
                Err(format!("Characteristic: {} not found", characteristic))?
            }
        } else {
            Err("You must be connected to write")?
        }

        Ok(())
    }

    async fn read(&mut self, _service: &str, characteristic: &str) -> Result<(), Box<dyn Error>> {
        if let Some(p) = &self.peripheral {
            p.discover_services().await.unwrap();

            let c = p
                .characteristics()
                .into_iter()
                .find(|c| c.uuid.to_string() == characteristic);

            if let Some(c) = c {
                println!("Reading characteristic {} ...", c.uuid);
                let content = p.read(&c).await?;
                println!("{:?}", content);
            } else {
                Err(format!("Characteristic: {} not found", characteristic))?
            }
        } else {
            Err("You must be connected to read")?
        }

        Ok(())
    }

    async fn notify(&mut self, _service: &str, characteristic: &str) -> Result<(), Box<dyn Error>> {
        if let Some(p) = &self.peripheral {
            let c = p
                .characteristics()
                .into_iter()
                .find(|c| c.uuid.to_string() == characteristic);

            if let Some(c) = c {
                if c.properties.contains(btleplug::api::CharPropFlags::NOTIFY) == false {
                    Err(format!(
                        "Characteristic {} doesn't have the notify attribute",
                        c.uuid.to_string()
                    ))?;
                }
                println!(
                    "Subscribing to characteristic {} notifications ...",
                    c.uuid.to_string()
                );

                p.subscribe(&c).await?;
                println!("OK");
                Ok(())
            } else {
                Err(format!("Characteristic {} not found", characteristic))?
            }
        } else {
            Err("You must be connected to notify")?
        }
    }

    async fn indicate(
        &mut self,
        _service: &str,
        characteristic: &str,
    ) -> Result<(), Box<dyn Error>> {
        if let Some(p) = &self.peripheral {
            let c = p
                .characteristics()
                .into_iter()
                .find(|c| c.uuid.to_string() == characteristic);

            if let Some(c) = c {
                if c.properties
                    .contains(btleplug::api::CharPropFlags::INDICATE)
                    == false
                {
                    Err(format!(
                        "Characteristic {} doesn't have the indicate attribute",
                        c.uuid.to_string()
                    ))?;
                }
                println!(
                    "Subscribing to characteristic {} indications ...",
                    c.uuid.to_string()
                );

                p.subscribe(&c).await?;
                println!("OK");
                Ok(())
            } else {
                Err(format!("Characteristic {} not found", characteristic))?
            }
        } else {
            Err("You must be connected to indicate")?
        }
    }

    async fn unsubscribe(
        &mut self,
        _service: &str,
        characteristic: &str,
    ) -> Result<(), Box<dyn Error>> {
        if let Some(p) = &self.peripheral {
            let c = p
                .characteristics()
                .into_iter()
                .find(|c| c.uuid.to_string() == characteristic);

            if let Some(c) = c {
                if c.properties.contains(btleplug::api::CharPropFlags::NOTIFY) == false
                    && c.properties
                        .contains(btleplug::api::CharPropFlags::INDICATE)
                        == false
                {
                    Err(format!(
                        "Characteristic {} doesn't have the notify or indicate attribute",
                        c.uuid.to_string()
                    ))?;
                }
                println!(
                    "Unsubscribing from characteristic {} notifications ...",
                    c.uuid.to_string()
                );
                p.unsubscribe(&c).await?;
                println!("OK");
                Ok(())
            } else {
                Err(format!("Characteristic {} not found", characteristic))?
            }
        } else {
            Err("You must be connected to notify")?
        }
    }

    async fn get_peripheral_infos(&self) -> Result<BlePeripheralInfo, Box<dyn Error>> {
        if let Some(p) = &self.peripheral {
            p.discover_services().await.unwrap();

            let services = p.services();
            let properties = p.properties().await?.unwrap();

            let mut infos = BlePeripheralInfo {
                periph_name: properties
                    .local_name
                    .unwrap_or_else(|| String::from("unknown")),
                periph_mac: self.get_address_or_uuid(&p).await?,
                rssi: properties.rssi.unwrap_or_else(|| 0),
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
            Ok(infos)
        } else {
            Err("You must be connected to get peripheral infos")?
        }
    }

    async fn connect(&mut self, uuid: &str) -> Result<(), Box<dyn Error>> {
        for p in &self.adapter.peripherals().await? {
            let properties = p.properties().await?.unwrap();
            let name = properties
                .local_name
                .unwrap_or_else(|| String::from("unknown"));

            if uuid == self.get_address_or_uuid(&p).await? {
                println!(
                    "Connecting to {} with uuid: {}",
                    name,
                    self.get_address_or_uuid(&p).await?
                );
                p.connect().await?;
                self.peripheral = Some(Box::new(p.clone()));
                self.notifications_thread_running
                    .store(true, atomic::Ordering::Relaxed);
                self.start_notifications_thread().await?;

                return Ok(());
            }
        }
        Err(format!("Peripheral with uuid {} not found", uuid))?
    }

    async fn disconnect(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(p) = &self.peripheral {
            let properties = p.properties().await?.unwrap();
            let name = properties
                .local_name
                .unwrap_or_else(|| String::from("unknown"));
            println!(
                "Disconnecting from {} with uuid: {} ... ",
                name,
                self.get_address_or_uuid(p).await?
            );
            self.notifications_thread_running
                .store(false, atomic::Ordering::Relaxed);
            p.disconnect().await?;
        } else {
            Err("You must be connected to disconnect")?
        }
        self.peripheral = None;
        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.peripheral.is_some()
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
                for (index, ad) in adapter_list.iter().enumerate() {
                    println!("[{}]: {:?}", index, ad);
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
            adapter: adapter.clone(),
            scan_list: Vec::new(),
            peripheral: None,
            notifications_thread_running: Arc::new(atomic::AtomicBool::new(false)),
        }
    }

    async fn get_address_or_uuid(
        &self,
        p: &btleplug::platform::Peripheral,
    ) -> Result<String, Box<dyn Error>> {
        let properties = p.properties().await?.unwrap();

        if cfg!(target_os = "macos") {
            return Ok(p.id().to_string());
        } else {
            return Ok(properties.address.to_string());
        }
    }

    async fn start_notifications_thread(&mut self) -> Result<(), Box<dyn Error>> {
        println!("Starting notifications thread");

        if let Some(p) = &self.peripheral {
            let mut notification_stream = p.notifications().await?;
            let atomic_is_running = self.notifications_thread_running.clone();

            thread::spawn(move || loop {
                if let Some(data) = block_on(notification_stream.next()) {
                    println!("Notification from [{:?}]: {:?}", data.uuid, data.value);
                }
                if !atomic_is_running.load(atomic::Ordering::Relaxed) {
                    println!("Stopping notifications thread");
                    return;
                }
                thread::sleep(Duration::from_millis(1));
            });
        }
        Ok(())
    }
}
