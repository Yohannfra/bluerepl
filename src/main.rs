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

    let bt: Box<dyn BleController> = match args.ble_lib {
        1 => Box::new(btleplug::BtleplugController::new().await),
        2 => Box::new(simpleble::SimpleBleController::new()),
        n => panic!("Unknown controller id {}", n),
    };

    let mut repl = Repl::new(bt);

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
    repl.start().await.expect("An error occured");
}
