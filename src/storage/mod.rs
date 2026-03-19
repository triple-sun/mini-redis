use std::error::Error;

pub trait Storage {
    fn set(&self, k: &str, v: &str);
    fn get(&self, k: &str) -> Result<String, Box<dyn Error + '_>>;
}

mod utils;
pub mod log_store;
pub mod mem_store;
