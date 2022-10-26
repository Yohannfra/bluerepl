use crate::controllers;

use std::error::Error;

use crate::controllers::BlePeripheralInfo;
use comfy_table::{Attribute, Cell, Table};

use crate::bluetooth_numbers::{characteristic_uuids, services_uuids};

fn print_gatt_infos(infos: &BlePeripheralInfo) {
    let mut table = Table::new();

    table.set_header(vec![
        Cell::new("Peripheral:").add_attribute(Attribute::Bold),
        Cell::new(&infos.periph_name),
    ]);

    table.add_row(vec!["Device address", &infos.periph_mac.to_string()]);
    table.add_row(vec!["RSSI", &format!("{}", infos.rssi,)]);

    table.add_row(vec![Cell::new("Service(s)").add_attribute(Attribute::Bold)]);

    for s in &infos.services {
        let fmt_service = format!(
            "{}\n{}\n{}",
            s.uuid,
            services_uuids::get_service_name_from_uuid(&s.uuid).unwrap_or_else(|| "".to_owned()),
            services_uuids::get_service_identifier_from_uuid(&s.uuid)
                .unwrap_or_else(|| "".to_owned())
        );
        let mut vec_service = vec!["UUID\nName\nIdentifier".to_owned(), fmt_service];

        for c in &s.characteriscics {
            vec_service[0].push_str("\n\nCharacteristic:\n");
            vec_service[0].push_str(" - UUID:\n - Name\n - Identifier\n - Properties");
            vec_service[1].push_str(&format!(
                "\n\n\n{}\n{}\n{}\n{:?}",
                c.uuid,
                characteristic_uuids::get_characteristic_name_from_uuid(&c.uuid)
                    .unwrap_or_else(|| "".to_owned()),
                characteristic_uuids::get_characteristic_identifier_from_uuid(&c.uuid)
                    .unwrap_or_else(|| "".to_owned()),
                c.properties
            ));
        }

        table.add_row(vec_service);
    }

    println!("{table}");
}

pub async fn gatt(bt: &mut dyn controllers::BleController) -> Result<(), Box<dyn Error>> {
    let infos: BlePeripheralInfo = bt.get_peripheral_infos().await?;
    print_gatt_infos(&infos);

    Ok(())
}

pub async fn adapter(bt: &mut dyn controllers::BleController) -> Result<(), Box<dyn Error>> {
    let infos = bt.get_adapter_infos().await?;
    println!("{}", infos);

    Ok(())
}
