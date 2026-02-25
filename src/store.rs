use std::collections::HashMap;

use crate::constants::NIL;

pub fn set<'a, 'b>(store: &mut HashMap<String, String>, k: &str, v: &str) {
    store.insert(k.to_string(), v.to_string());
}

pub fn get<'a>(store: &'a HashMap<String, String>, k: &'a str) -> &'a str {
    match store.get(k) {
        None => NIL,
        Some(value) => value,
    }
}
