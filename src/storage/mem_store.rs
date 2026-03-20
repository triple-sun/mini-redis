use std::{
    collections::HashMap,
    error::Error,
    sync::{Arc, RwLock},
};

use crate::{
    config::Config,
    storage::Storage,
    utils::{prepare_key, print_err},
};

pub struct MemStorage {
    options: Config,
    pub db: RwLock<HashMap<String, String>>,
}

impl MemStorage {
    pub fn init(options: Config) -> Arc<MemStorage> {
        Arc::new(MemStorage {
            db: Default::default(),
            options,
        })
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
}

impl Storage for MemStorage {
    fn set(&self, k: &str, v: &str) {
        self.db_set(k, v);
    }

    fn get(&self, k: &str) -> Result<String, Box<dyn Error + '_>> {
        self.db_get(k)
    }
}
