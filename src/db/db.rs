use core::panic;
use std::fs::{File, OpenOptions};

const FILE_PATH: &str = "resources/quotes.json";

pub fn open_file(write: bool) -> File {
    let file = if write {
        OpenOptions::new()
            .append(true)
            .create(true)
            .open(FILE_PATH)
            .unwrap_or_else(|e| {
                panic!(
                    "Error opening file for writing: {}. Error: {}",
                    FILE_PATH, e
                )
            })
    } else {
        OpenOptions::new()
            .read(true)
            .open(FILE_PATH)
            .unwrap_or_else(|e| {
                panic!(
                    "Error opening file for reading: {}. Error: {}",
                    FILE_PATH, e
                )
            })
    };
    file
}
