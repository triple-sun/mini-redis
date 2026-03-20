use std::{
    collections::HashMap,
    error::Error,
    fs::OpenOptions,
    io::{self, BufRead, BufReader, BufWriter, Write},
    sync::{Arc, RwLock},
    thread::{self, sleep},
    time::Duration,
};

use atomic_write_file::AtomicWriteFile;
use colored::Colorize;

use crate::{
    command::Command,
    config::Config,
    storage::Storage,
    utils::{prepare_key, print_err},
};

pub struct LogStorage {
    db: RwLock<HashMap<String, String>>,
    options: Config,
}

impl LogStorage {
    pub fn init(options: Config) -> Arc<LogStorage> {
        let mut store = LogStorage {
            options,
            db: Default::default(),
        };

        if let Err(err) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&store.options.log_file_path)
        {
            panic!("Could not upsert log file: {err}");
        };

        match store.restore_from_log() {
            Err(err) => print_err(format!("Could not restore from log file: {err}")),
            Ok(count) => println!(
                "{}",
                format!("\n{count} lines restored from log file!")
                    .blue()
                    .bold()
            ),
        }

        let store = Arc::new(store);
        let compact_store = Arc::clone(&store);

        thread::spawn(move || {
            loop {
                sleep(Duration::new(compact_store.options.compact_interval, 0));
                if let Err(err) = compact_store.compact_log() {
                    print_err(format!("Could not compact store: {err}"));
                }
            }
        });

        return store;
    }

    fn db_set(&self, k: &str, v: &str) {
        let k = prepare_key(self.options.ignore_case, k);

        let mut db_lock = match self.db.write() {
            Ok(db) => db,
            Err(msg) => {
                print_err(format!("Error locking db: {msg}"));
                return;
            }
        };

        db_lock.insert(k, v.to_string());
    }

    fn db_get(&self, k: &str) -> Result<String, Box<dyn Error + '_>> {
        let db_lock = self.db.read()?;

        if let Some(v) = db_lock.get(k) {
            return Ok(v.to_owned());
        }

        Ok("(nil)".to_string())
    }

    pub fn save_to_log(&self, k: &str, v: &str) -> Result<(), io::Error> {
        let file = OpenOptions::new()
            .append(true)
            .open(&self.options.log_file_path)?;

        writeln!(BufWriter::new(&file), "SET {k} {v}")?;

        Ok(())
    }

    pub fn restore_from_log(&mut self) -> Result<u32, Box<dyn Error>> {
        let mut count: u32 = 0;

        let log_file = OpenOptions::new()
            .read(true)
            .open(&self.options.log_file_path)?;

        for line in BufReader::new(&log_file).lines() {
            let line = line?;
            let command = Command::from(&line)?;

            match command {
                Command::Set(k, v) => {
                    self.db_set(k, v);
                    count += 1;
                }
                _ => print_err("Expected SET command, got: {command}".to_string()),
            };
        }

        Ok(count)
    }

    pub fn compact_log(&self) -> Result<u32, Box<dyn Error + '_>> {
        let mut count: u32 = 0;
        let mut log_file = AtomicWriteFile::options().open(&self.options.log_file_path)?;

        for (k, v) in self.db.read()?.iter() {
            writeln!(log_file, "SET {k} {v}")?;
            count += 1;
        }

        log_file.commit()?;

        Ok(count)
    }
}

impl Storage for LogStorage {
    fn set(&self, k: &str, v: &str) {
        self.db_set(k, v);

        if let Err(err) = self.save_to_log(k, v) {
            print_err(format!("Could not save command to log: {err}"));
        }
    }

    fn get(&self, k: &str) -> Result<String, Box<dyn Error + '_>> {
        self.db_get(k)
    }
}
