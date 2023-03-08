use std::{thread, time};

pub fn run(time_ms: u64) {
    thread::sleep(time::Duration::from_millis(time_ms));
}
