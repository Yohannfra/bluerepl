extern crate exitcode;

#[macro_use]
extern crate lazy_static;

mod bluetooth_numbers;
mod controllers;
mod preset;
mod repl;
mod utils;

use clap::Parser;

use controllers::btleplug;
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

    /// autoconnect to peripheral described in preset
    #[clap(short, long)]
    autoconnect: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("bluerepl Version: {}", env!("CARGO_PKG_VERSION"));

    let args = Args::parse();

    let mut bt = match args.ble_lib.as_str() {
        "btleplug" => btleplug::BtleplugController::new().await,
        "simpleble" => todo!("simpleble support is not yet implemented"),
        "bleuio" => todo!("bleuio support is not yet implemented"),
        n => panic!("Unknown controller id {}", n),
    };

    let mut repl = Repl::new(&mut bt).await;

    if let Some(preset_file) = args.preset_file {
        let mut pr = Preset::new(preset_file).unwrap();

        if args.autoconnect {
            if !pr.is_autoconnect_possible() {
                panic!("A name or an address must be in the preset file to use the autoconnect feature");
            }
            pr.device.as_mut().unwrap().autoconnect = Some(true);
        }
        repl.set_preset(pr);
    } else if args.autoconnect {
        panic!("-a --autoconnect can't be used without preset");
    }

    repl.start().await
}
