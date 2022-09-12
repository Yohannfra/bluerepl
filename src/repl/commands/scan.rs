use crate::controllers;

use controllers::BlePeripheral;

use comfy_table::Table;

use std::error::Error;

pub fn print_scan_list(list: &Vec<BlePeripheral>, show_all: bool) {
    let mut table = Table::new();

    table.add_row(vec!["Index", "Name", "UUID", "RSSI"]);

    let mut index = 0;
    for p in list {
        if show_all == false && p.name == "unknown" {
            continue;
        }
        table.add_row(vec![
            &index.to_string(),
            &p.name,
            &p.address_uuid,
            &p.rssi.to_string(),
        ]);
        index += 1;
    }

    if index == 0 {
        println!("Empty scan list");
        return;
    }

    println!("{table}");
}

pub async fn run(
    bt: &mut Box<dyn controllers::BleController>,
    timeout: u32,
    show_all: bool,
) -> Result<Vec<BlePeripheral>, Box<dyn Error>> {
    let res = bt.scan(timeout).await?;

    print_scan_list(&res, show_all);

    Ok(res)
}
