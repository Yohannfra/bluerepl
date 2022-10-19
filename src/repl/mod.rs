use rustyline::error::ReadlineError;
use rustyline::Editor;

mod cli;

mod commands;

use crate::controllers;
use crate::preset::Preset;
use controllers::BleController;

use std::error::Error;

pub struct Repl {
    bt: Box<dyn BleController>,
    editor: Editor<()>,
    preset: Option<Preset>,
}

const HISTORY_FP: &str = ".history.txt";

impl Repl {
    pub fn new(bt: Box<dyn controllers::BleController>) -> Repl {
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
                return line;
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
                println!("{:?}", mt);
            }

            Some(("read", mt)) => {
                println!("{:?}", mt);
            }

            Some(("scan", mt)) => {
                let show_all = mt.contains_id("all");

                if mt.contains_id("list") {
                    return commands::scan::print_scan_list(&self.bt.get_scan_list(), show_all);
                }

                let timeout = *mt.get_one::<usize>("timeout").unwrap();

                commands::scan::run(&mut self.bt, timeout, show_all).await?;
            }

            Some(("info", mt)) => {
                match mt.subcommand_name() {
                    Some("adapter") => commands::info::adapter(&mut self.bt).await?,
                    Some("gatt") => {
                        if self.bt.is_connected() == false {
                            Err("You must be connected to a peripheral to run this command")?;
                        }
                        commands::info::gatt(&mut self.bt).await?;
                    },
                    Some("preset") => {
                        if self.preset.is_some() {
                            self.preset.as_ref().unwrap().print();
                        } else {
                            Err("No preset loaded")?;
                        }
                    },
                    _ => panic!("Code should never be here"),
                }
            }

            Some(("connect", mt)) => {
                if mt.contains_id("name") {
                    let name = mt.get_one::<String>("name").unwrap();
                    commands::connect::by_name(&mut self.bt, name).await?;
                } else if mt.contains_id("mac") {
                    let addr = mt.get_one::<String>("mac").unwrap();
                    commands::connect::by_address(&mut self.bt, addr).await?;
                } else if mt.contains_id("id") {
                    let index = *mt.get_one::<usize>("id").unwrap();
                    commands::connect::by_index(&mut self.bt, index).await?;
                } else {
                    let identifier = mt.get_one::<String>("identifier").unwrap();
                    commands::connect::auto_detect_identifier(&mut self.bt, &identifier).await?;
                }
            }

            Some(("disconnect", _mt)) => {
                if self.bt.is_connected() == false {
                    Err("You must be connected to a peripheral to run this command")?;
                } else {
                    commands::disconnect::run(&mut self.bt).await?;
                }
            }

            Some(("indicate", mt)) => {
                println!("{:?}", mt);
            }

            Some(("notify", mt)) => {
                println!("{:?}", mt);
            }

            Some(("unsubscribe", mt)) => {
                println!("{:?}", mt);
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
