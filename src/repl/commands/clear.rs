pub fn run() {
    if cfg!(windows) {
        std::process::Command::new("cls").status().unwrap();
    } else {
        std::process::Command::new("clear").status().unwrap();
    }
}
