extern crate exitcode;

mod controllers;
mod preset;
mod repl;
mod utils;

use clap::Parser;
use controllers::btleplug;
use controllers::BleController;
use preset::Preset;
use repl::Repl;
use std::error::Error;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the preset file to load
    preset_file: Option<std::path::PathBuf>,

    #[clap(short, default_value = "btleplug")]
    /// Ble lib to use :
    /// - btleplug
    /// - simpleble
    /// - bleuio
    ble_lib: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("BlueREPL Version: {}", env!("CARGO_PKG_VERSION"));

    let args = Args::parse();

    let bt: Box<dyn BleController> = match args.ble_lib.as_str() {
        "btleplug" => Box::new(btleplug::BtleplugController::new().await),
        // "simpleble" => Box::new(simpleble::SimpleBleController::new()),
        // "bleuio" => Box::new(simpleble::BleuIOController::new()),
        n => panic!("Unknown controller id {}", n),
    };

    let mut repl = Repl::new(bt);

    if args.preset_file != None {
        let pr = match Preset::new(args.preset_file.unwrap()) {
            Ok(p) => p,
            Err(e) => {
                panic!("{}", e);
            }
        };
        pr.print();
        repl.set_preset(pr);
    }
    repl.start().await
}
