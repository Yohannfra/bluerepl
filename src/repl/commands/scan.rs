use crate::controllers;

use controllers::BlePeripheral;

use comfy_table::Table;

pub fn print_scan_list(list: &Vec<BlePeripheral>, show_all: bool) {
    let mut table = Table::new();

    table.add_row(vec!["Index", "Name", "Mac"]);

    let mut index = 0;
    for p in list {
        if show_all == false && p.name == "unknown" {
            continue;
        }
        table.add_row(vec![&index.to_string(), &p.name, &p.mac_addr]);
        index += 1;
    }

    if index == 0 {
        println!("Empty scan list");
        return;
    }

    println!("{table}");
}

pub async fn run(bt: &mut Box<dyn controllers::BleController>, timeout: u32, show_all: bool) -> Vec<BlePeripheral> {
    let res = bt.scan(timeout).await;

    if res.is_err() {
        eprintln!("{:?}", res.unwrap_err());
        return Vec::new();
    }

    let list = res.unwrap();

    print_scan_list(&list, show_all);

    list
}
