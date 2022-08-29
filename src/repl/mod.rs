use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};

use crate::utils;

use crate::controllers;

pub struct Repl {
    last_exit_code: i32,
    bt: Option<Box<dyn controllers::BleController>>,
    editor: Editor<()>,
}

const HISTORY_FP: &str = ".history.txt";

impl Repl {
    pub fn new() -> Repl {
        Repl {
            last_exit_code: 0,
            bt: None,
            editor: Editor::<()>::new().unwrap(),
        }
    }

    pub fn set_ble_controller(&mut self, ctl: &mut dyn controllers::BleController) {
        // self.bt = Some(Box::new(ctl));
    }

    fn get_line(&mut self) -> String {
        let readline = self.editor.readline(">>");
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

    pub fn start(&mut self) -> Result<()> {
        if self.editor.load_history(HISTORY_FP).is_err() {
            println!("No previous history.");
        }

        loop {
            let line = self.get_line();
            let sp: Vec<String> = utils::split_line_in_args(line);
            if sp.is_empty() {
                continue;
            }

            // let command = sp[0];

            println!("{:?}", sp);
        }
    }
}
