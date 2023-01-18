use crate::error::*;
use chrono;
use std::{fs::File, io::Write, path::PathBuf};

#[derive(Clone, PartialEq, Debug, Copy)]
pub struct Logger<'a> {
    name: &'a str,
    timestamp: bool,       // defualt: true
    file: Option<&'a PathBuf>, // default: None
}

impl Logger<'_> {
    /// Create a new logger with the defualt values
    pub fn new(name: &'static str) -> Self {
        let logger = Logger {
            name,
            timestamp: true,
            file: None
        };
        logger
    }

    /// Set the timestamp value
    pub fn timestamp(mut self, toggle_timetamp: bool) -> Self {
        self.timestamp = toggle_timetamp;
        self
    }

    /// Set the output file
    pub fn file(mut self, file: &'static PathBuf) -> Self {
        self.file = Some(file);
        self
    }

    pub fn print(self, message: String) -> Result<(), Error> {
        let timestamp = match self.timestamp {
            true => format!("{} ", chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S%.3f")),
            _ => String::from(""),
        };

        let m = format!("[{}{}] {}", timestamp, self.name, message);
        
        // Print
        println!("{m}");

        // Write to file 
        if let Some(path) = self.file {
            let mut f = match File::create(path) {
                Ok(x) => x,
                Err(x) => return Err(Error::new(x.to_string(), 1).fatal()),
            };

            match writeln!(f, "{}", m) {
                Err(x) => return Err(Error::new(x.to_string(), 1).fatal()),
                _ => (),
            };
        }

        Ok(())
    }
}
