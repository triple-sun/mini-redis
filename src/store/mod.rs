use std::{
    collections::HashMap,
    error::Error,
    fs::OpenOptions,
    io::{self, BufRead, BufReader, BufWriter, Write},
    sync::RwLock,
};

use atomic_write_file::AtomicWriteFile;

use crate::command::{Command, parse_input};

const LOG_FILE_PATH: &str = "./db_log/db.log";

pub struct Store {
    pub db: RwLock<HashMap<String, String>>,
    pub log_file_path: String,
}

impl Store {
    pub fn init() -> Store {
        let mut store = Store {
            db: Default::default(),
            log_file_path: LOG_FILE_PATH.to_string(),
        };

        if let Err(err) = OpenOptions::new().create(true).open(&store.log_file_path) {
            panic!("Could not upsert log file: {err}");
        };

        if let Err(err) = store.restore_from_log() {
            eprintln!("Could not restore from log file: {err}");
        };

        return store;
    }

    pub fn set(&self, k: &str, v: &str) {
        let mut db_lock = match self.db.write() {
            Ok(db) => db,
            Err(msg) => {
                eprintln!("Error locking db: {msg}");
                return;
            }
        };

        db_lock.insert(k.to_string(), v.to_string());
    }

    pub fn get(&self, k: &str) -> Result<String, Box<dyn Error + '_>> {
        let db_lock = self.db.read()?;

        if let Some(v) = db_lock.get(k) {
            return Ok(v.to_owned());
        }

        Ok("(nil)".to_string())
    }

    pub fn save_to_log(&self, k: &str, v: &str) -> Result<(), io::Error> {
        let file = OpenOptions::new().append(true).open(&self.log_file_path)?;

        writeln!(BufWriter::new(&file), "SET {k} {v}")?;

        Ok(())
    }

    pub fn restore_from_log(&mut self) -> Result<(), Box<dyn Error>> {
        let log_file = OpenOptions::new()
            .read(true)
            .open(self.log_file_path.clone())?;

        for line in BufReader::new(&log_file).lines() {
            let line = line?;
            let command = parse_input(&line)?;

            match command {
                Command::Set(k, v) => self.set(k, v),
                _ => println!("Expected SET command, got: {command}"),
            }
        }

        Ok(())
    }

    pub fn compact_log(&self) -> Result<(), Box<dyn Error + '_>> {
        let mut log_file = AtomicWriteFile::options().open(LOG_FILE_PATH)?;

        for (k, v) in self.db.read()?.iter() {
            writeln!(log_file, "SET {k} {v}")?;
        }

        log_file.commit()?;

        Ok(())
    }
}
