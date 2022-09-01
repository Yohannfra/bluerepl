extern crate exitcode;

mod controllers;
mod preset;
mod repl;
mod utils;

use clap::Parser;
use controllers::BleController;
use controllers::{btleplug, simpleble};
use preset::Preset;
use repl::Repl;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the preset file to load
    preset_file: Option<std::path::PathBuf>,

    #[clap(short, default_value_t = 1)]
    /// Ble lib to use
    /// 1=btleplug
    /// 2=simpleble
    ble_lib: u8,
}

#[tokio::main]
async fn main() {
    println!("BlueREPL Version: {}", env!("CARGO_PKG_VERSION"));

    let args = Args::parse();

    let mut repl = Repl::new();

    if args.preset_file != None {
        let pr = match Preset::new(args.preset_file.unwrap()) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        };
        pr.print();
    }

    // TODO
    let bt = controllers::btleplug::BtleplugController::new().await;
    // match args.ble_lib {
    //     1 => bt = dyn controllers::btleplug::BtleplugController::new(),
    //     2 => bt = dyn controllers::simpleble::SimpleBleController::new(),
    //     n => panic!("Unknown controller id {}", n),
    // };

    repl.start().expect("An error occured");
}
