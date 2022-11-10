use rustyline::error::ReadlineError;
use rustyline::Editor;

mod cli;

pub mod commands;

use crate::controllers;
use crate::preset::Preset;
use controllers::BleController;
use std::error::Error;

pub struct Repl<'a> {
    bt: &'a mut dyn BleController,
    editor: Editor<()>,
    preset: Option<Preset>,
}

const HISTORY_FP: &str = ".bluerepl_history.txt";

impl Repl<'_> {
    pub async fn new(bt: &mut dyn BleController) -> Repl {
        Repl {
            bt,
            editor: Editor::<()>::new().unwrap(),
            preset: None,
        }
    }

    pub fn set_preset(&mut self, pr: Preset) {
        self.preset = Some(pr);
    }

    fn get_line(&mut self) -> String {
        let readline = self.editor.readline(">> ");
        match readline {
            Ok(line) => {
                self.editor.add_history_entry(line.as_str());
                self.editor.save_history(HISTORY_FP).unwrap();
                line
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C, bye");
                std::process::exit(exitcode::OSERR);
            }
            Err(ReadlineError::Eof) => {
                println!("EOF, bye");
                std::process::exit(exitcode::OK);
            }
            Err(err) => {
                panic!("{}", err)
            }
        }
    }

    fn try_replacing_service_and_characteristics_with_preset_defs(
        &self,
        service: &mut String,
        characteristic: &mut String,
    ) {
        if let Some(preset) = &self.preset {
            if let Some(uuid) = preset.get_service_uuid_from_name(service) {
                *service = uuid;
            }
            if let Some(uuid) = preset.get_characteristic_uuid_from_name(service, characteristic) {
                *characteristic = uuid;
            }
        }
    }

    async fn execute_command(&mut self, matches: clap::ArgMatches) -> Result<(), Box<dyn Error>> {
        match matches.subcommand() {
            Some(("quit", _)) => {
                println!("EOF, bye");
                std::process::exit(exitcode::OK);
            }

            Some(("clear", _)) => {
                commands::clear::run();
            }

            Some(("write", mt)) => {
                let mut service = mt.get_one::<String>("service").unwrap().clone();
                let mut characteristic = mt.get_one::<String>("characteristic").unwrap().clone();
                let payload = mt.get_one::<String>("payload").unwrap();
                let response: bool = mt.contains_id("resp");

                self.try_replacing_service_and_characteristics_with_preset_defs(
                    &mut service,
                    &mut characteristic,
                );

                commands::write::write(self.bt, &service, &characteristic, payload, response)
                    .await?;
            }

            Some(("read", mt)) => {
                let mut service = mt.get_one::<String>("service").unwrap().clone();
                let mut characteristic = mt.get_one::<String>("characteristic").unwrap().clone();
                let format = mt.get_one::<String>("format").unwrap();

                self.try_replacing_service_and_characteristics_with_preset_defs(
                    &mut service,
                    &mut characteristic,
                );

                commands::read::read(self.bt, &service, &characteristic, format).await?;
            }

            Some(("scan", mt)) => {
                let show_all = mt.contains_id("all");

                if mt.contains_id("list") {
                    return commands::scan::print_scan_list(&self.bt.get_scan_list(), show_all);
                }

                let timeout = *mt.get_one::<usize>("timeout").unwrap();

                commands::scan::run(self.bt, timeout, true, show_all).await?;
            }

            Some(("info", mt)) => match mt.subcommand_name() {
                Some("adapter") => commands::info::adapter(self.bt).await?,
                Some("gatt") => {
                    if !self.bt.is_connected() {
                        Err("You must be connected to a peripheral to run this command")?;
                    }
                    commands::info::gatt(self.bt, &self.preset).await?;
                }
                _ => panic!("Code should never be here"),
            },

            Some(("connect", mt)) => {
                if mt.contains_id("name") {
                    let name = mt.get_one::<String>("name").unwrap();
                    commands::connect::by_name(self.bt, name).await?;
                } else if mt.contains_id("mac") {
                    let addr = mt.get_one::<String>("mac").unwrap();
                    commands::connect::by_address(self.bt, addr).await?;
                } else if mt.contains_id("id") {
                    let index = *mt.get_one::<usize>("id").unwrap();
                    commands::connect::by_index(self.bt, index).await?;
                } else {
                    let identifier = mt.get_one::<String>("identifier").unwrap();
                    commands::connect::auto_detect_identifier(self.bt, identifier).await?;
                }
            }

            Some(("disconnect", _mt)) => {
                if !self.bt.is_connected() {
                    Err("You must be connected to a peripheral to run this command")?;
                } else {
                    commands::disconnect::run(self.bt).await?;
                }
            }

            Some(("indicate", mt)) => {
                let mut service = mt.get_one::<String>("service").unwrap().clone();
                let mut characteristic = mt.get_one::<String>("characteristic").unwrap().clone();
                let format = mt.get_one::<String>("format").unwrap();

                self.try_replacing_service_and_characteristics_with_preset_defs(
                    &mut service,
                    &mut characteristic,
                );

                commands::indicate::indicate(self.bt, &service, &characteristic, format).await?;
            }

            Some(("notify", mt)) => {
                let mut service = mt.get_one::<String>("service").unwrap().clone();
                let mut characteristic = mt.get_one::<String>("characteristic").unwrap().clone();
                let format = mt.get_one::<String>("format").unwrap();

                self.try_replacing_service_and_characteristics_with_preset_defs(
                    &mut service,
                    &mut characteristic,
                );

                commands::notify::notify(self.bt, &service, &characteristic, format).await?;
            }

            Some(("unsubscribe", mt)) => {
                let mut service = mt.get_one::<String>("service").unwrap().clone();
                let mut characteristic = mt.get_one::<String>("characteristic").unwrap().clone();

                self.try_replacing_service_and_characteristics_with_preset_defs(
                    &mut service,
                    &mut characteristic,
                );

                commands::unsubscribe::unsubscribe(self.bt, &service, &characteristic).await?;
            }

            Some(("preset", mt)) => {
                if self.preset.is_none() {
                    Err("No preset loaded")?;
                }

                if mt.subcommand().is_none() {
                    self.preset.as_ref().unwrap().print();
                } else {
                    match mt.subcommand() {
                        Some(("command", arg)) => {
                            let command_name = arg.get_one::<String>("command_name").unwrap();
                            self.preset
                                .as_ref()
                                .unwrap()
                                .run_command(self.bt, command_name)
                                .await?;
                        }
                        Some(("function", arg)) => {
                            let function_name = arg.get_one::<String>("function_name").unwrap();
                            self.preset
                                .as_ref()
                                .unwrap()
                                .run_function(self.bt, function_name)
                                .await?;
                        }
                        _ => (),
                    }
                }
            }

            _ => {
                eprintln!("Unknown command: '{:?}'", matches);
            }
        }
        Ok(())
    }

    pub async fn start(&mut self) -> ! {
        if self.editor.load_history(HISTORY_FP).is_err() {
            println!("No previous history.");
        }

        if let Some(preset) = &self.preset {
            if preset.should_autoconnect() {
                println!("autoconnect found in preset, running auto connection");
                self.preset
                    .as_ref()
                    .unwrap()
                    .autoconnect(self.bt)
                    .await
                    .unwrap();
            }
        }

        loop {
            let line = self.get_line();

            if line.trim().is_empty() {
                continue;
            }

            let args = match shlex::split(&line).ok_or("Parsing error: Invalid quoting") {
                Ok(a) => a,
                Err(e) => {
                    eprintln!("{}", e);
                    continue;
                }
            };

            let matches = cli::cli().try_get_matches_from(&args);

            if matches.is_err() {
                println!("{}", matches.unwrap_err());
                continue;
            } else {
                match self.execute_command(matches.unwrap()).await {
                    Ok(_) => (),
                    Err(e) => eprintln!("{}", e),
                }
            }
        }
    }
}
