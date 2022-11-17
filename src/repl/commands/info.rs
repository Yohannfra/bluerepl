use crate::controllers;

use std::error::Error;

use crate::controllers::BlePeripheralInfo;
use crate::repl::commands;
use comfy_table::{Attribute, Cell, Table};

use crate::bluetooth_numbers::{characteristic_uuids, services_uuids};
use crate::Preset;

async fn print_gatt_infos(
    bt: &mut dyn controllers::BleController,
    infos: &BlePeripheralInfo,
    p: &Option<Preset>,
) {
    let mut table = Table::new();

    table.set_header(vec![
        Cell::new("Peripheral:").add_attribute(Attribute::Bold),
        Cell::new(&infos.periph_name),
    ]);

    table.add_row(vec!["Device address", &infos.periph_mac.to_string()]);
    table.add_row(vec!["RSSI", &format!("{}", infos.rssi,)]);

    table.add_row(vec![Cell::new("Service(s)").add_attribute(Attribute::Bold)]);

    for s in &infos.services {
        let mut str_service: String = "UUID".to_owned();
        let mut fmt_service: String = s.uuid.to_string();

        // Service name
        if let Some(name) = services_uuids::get_service_name_from_uuid(&s.uuid, p) {
            fmt_service.push_str(&format!("\n{}", name));
            str_service.push_str("\nName");
        }

        // Service identifier
        if let Some(identifier) = services_uuids::get_service_identifier_from_uuid(&s.uuid) {
            fmt_service.push_str(&format!("\n{}", identifier));
            str_service.push_str("\nIdentifier");
        }

        let mut vec_service: Vec<String> = vec![str_service, fmt_service];

        for c in &s.characteriscics {
            vec_service[0].push_str("\n\nCharacteristic:\n");
            vec_service[0].push_str(" - UUID:\n - Properties");
            vec_service[1].push_str(&format!("\n\n\n{}\n{:?}", c.uuid, c.properties));

            // Characteristic name
            if let Some(name) =
                characteristic_uuids::get_characteristic_name_from_uuid(&s.uuid, &c.uuid, p)
            {
                vec_service[0].push_str("\n - Name");
                vec_service[1].push_str(&format!("\n{}", name))
            }

            // Characteristic identifier
            if let Some(identifier) =
                characteristic_uuids::get_characteristic_identifier_from_uuid(&c.uuid)
            {
                vec_service[0].push_str("\n - Identifier");
                vec_service[1].push_str(&format!("\n{}", identifier))
            }

            // Characteristic value
            if c.properties
                .contains(controllers::CharacteristicProperties::READ)
            {
                let mut val_as_str = String::new();

                if let Some(identifier) =
                    characteristic_uuids::get_characteristic_identifier_from_uuid(&c.uuid)
                {
                    if identifier.contains("string") {
                        val_as_str = commands::read::read_as_str(bt, &s.uuid, &c.uuid, "text")
                            .await
                            .unwrap();
                    }
                }

                if val_as_str.is_empty() {
                    val_as_str = commands::read::read_as_str(bt, &s.uuid, &c.uuid, "hex")
                        .await
                        .unwrap();

                    if val_as_str.len() > 45 {
                        val_as_str = val_as_str[0..50].to_owned() + " ...";
                    }
                }

                vec_service[0].push_str("\n - Value");
                vec_service[1].push_str(&format!("\n{}", val_as_str))
            }
        }
        table.add_row(vec_service);
    }

    println!("{table}");
}

pub async fn gatt(
    bt: &mut dyn controllers::BleController,
    p: &Option<Preset>,
) -> Result<(), Box<dyn Error>> {
    let infos: BlePeripheralInfo = bt.get_peripheral_infos().await?;
    print_gatt_infos(bt, &infos, p).await;

    Ok(())
}

pub async fn adapter(bt: &mut dyn controllers::BleController) -> Result<(), Box<dyn Error>> {
    let infos = bt.get_adapter_infos().await?;
    println!("{}", infos);

    Ok(())
}
