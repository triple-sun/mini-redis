use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufRead, BufReader, BufWriter, Write},
};

use crate::{
    command::{Command, parse_input},
    constants::{LOG_FILE_PATH, NIL},
};

fn upsert_log_file() -> File {
    let db_log = fs::OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(LOG_FILE_PATH);

    match db_log {
        Err(msg) => panic!("Could not open log file at {LOG_FILE_PATH}: {msg}"),
        Ok(file) => return file,
    }
}

pub struct Store {
    pub db: HashMap<String, String>,
    pub log: File,
}

impl Default for Store {
    fn default() -> Self {
        Self {
            db: Default::default(),
            log: upsert_log_file(),
        }
    }
}

impl Store {
    pub fn init() -> Store {
        Store::default()
    }

    pub fn set(&mut self, k: &str, v: &str) {
        self.db.insert(k.to_string(), v.to_string());
    }

    pub fn get(&self, k: &str) -> &str {
        match self.db.get(k) {
            None => NIL,
            Some(value) => value,
        }
    }

    pub fn save(&self, k: &str, v: &str) {
        let file = &self.log;
        let mut writer = BufWriter::new(file);

        if let Err(msg) = writeln!(&mut writer, "SET {k} {v}") {
            eprintln!("Could not write SET {k} {v} to log: {msg}")
        }
    }

    pub fn restore(&mut self) {
        for line in BufReader::new(&self.log).lines() {
            let line = match line {
                Ok(line) => line,
                Err(msg) => {
                    eprintln!("Failed to read log line: {msg}");
                    continue;
                }
            };

            match parse_input(&line) {
                Err(msg) => panic!("Error while reading log file: {msg}"),
                Ok(command) => match command {
                    Command::Set(k, v) => {
                        self.db.insert(k.to_string(), v.to_string());
                    }
                    _ => eprintln!("Expected SET command, got: {command}"),
                },
            }
        }
    }
}
