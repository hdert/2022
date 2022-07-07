extern crate chrono;

use std::fs::{File, OpenOptions};
use std::io;
use std::io::Write;

use chrono::*;

fn main() {
    let local: DateTime<Local> = Local::now();
    println!("{}", local);
    match log_time("log.txt") {
        Ok(_) => println!("File created."),
        Err(_) => println!("Error"),
    }
}

fn log_time(filename: &'static str) -> io::Result<()> {
    let entry = log_time_entry();
    let bytes = entry.as_bytes();

    record_entry_in_log(filename, &bytes);
    Ok(())
}

fn record_entry_in_log(filename: &str, bytes: &[u8]) -> io::Result<()> {
    let mut f = OpenOptions::new()
        .append(true)
        .write(true)
        .create(true)
        .open(filename)
        .unwrap();
    f.write_all(bytes);
    Ok(())
}

fn log_time_entry() -> String {
    let local: DateTime<Local> = Local::now();
    local.format("%a, %b %d %Y %I:%M:%S %p\n").to_string()
}
