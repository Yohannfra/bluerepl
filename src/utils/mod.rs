pub mod print_bytes;

use std::io::Write;

pub fn get_usize_input(prompt: &str) -> usize {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();

    std::io::stdin()
        .lines()
        .next()
        .expect("stdin should be available")
        .expect("couldn't read from stdin")
        .trim()
        .parse()
        .expect("input was not an integer")
}
