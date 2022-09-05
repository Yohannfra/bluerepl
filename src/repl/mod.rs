use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};

mod cli;

mod commands;

use crate::controllers;
use controllers::BleController;

pub struct Repl {
    bt: Box<dyn BleController>,
    editor: Editor<()>,
}

const HISTORY_FP: &str = ".history.txt";

impl Repl {
    pub fn new(bt: Box<dyn controllers::BleController>) -> Repl {
        Repl {
            bt,
            editor: Editor::<()>::new().unwrap(),
        }
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
                println!("Error: {:?}", err);
                std::process::exit(exitcode::DATAERR);
            }
        }
    }

    pub async fn start(&mut self) -> Result<()> {
        if self.editor.load_history(HISTORY_FP).is_err() {
            println!("No previous history.");
        }

        let mut scan_list: Vec<controllers::BlePeripheral> = Vec::new();

        loop {
            let line = self.get_line();

            if line.trim().is_empty() {
                continue;
            }

            let args = shlex::split(&line).ok_or("error: Invalid quoting").unwrap();
            println!("{:?}", args);

            let matches = cli::cli().try_get_matches_from(&args);

            if matches.is_err() {
                println!("{}", matches.unwrap_err());
                continue;
            }

            match matches.unwrap().subcommand() {
                Some(("quit", _)) => {
                    println!("EOF, bye");
                    std::process::exit(exitcode::OK);
                }

                Some(("clear", _)) => {
                    if cfg!(windows) {
                        std::process::Command::new("cls").status().unwrap();
                    } else {
                        std::process::Command::new("clear").status().unwrap();
                    }
                }

                Some(("write", mt)) => {
                    println!("{:?}", mt);
                }

                Some(("read", mt)) => {
                    println!("{:?}", mt);
                }

                Some(("scan", mt)) => {
                    let timeout: u32;

                    let show_all = mt.is_present("all");

                    if mt.is_present("list") {
                        commands::scan::print_scan_list(&scan_list, show_all);
                        continue;
                    }

                    match mt.get_one::<String>("timeout").unwrap().parse::<u32>() {
                        Ok(n) => timeout = n,
                        Err(e) => {
                            eprintln!("{}", e);
                            continue;
                        }
                    };

                    scan_list = commands::scan::run(&mut self.bt, timeout, show_all).await;
                }

                Some(("info", mt)) => {
                    println!("{:?}", mt);
                }

                Some(("connect", mt)) => {
                    println!("{:?}", mt);
                }

                Some(("disconnect", mt)) => {
                    println!("{:?}", mt);
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

                // Some((name, _matches)) => unimplemented!("{}", name),
                // None => unreachable!("subcommand required"),
                _ => {
                    eprintln!("Unknown command: '{}'", &line);
                }
            }
        }
    }
}
