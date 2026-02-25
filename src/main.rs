use std::{
    collections::HashMap,
    io::{self},
};

use crate::{
    command::{Command, parse_input},
    store::{get, set},
};

mod command;
mod constants;
mod errors;
mod store;

fn main() {
    println!(
        "Welcome to mini-redis!\nPlease input your command\nSupported commands:\n`EXIT`, `exit` to exit;"
    );

    let mut store: HashMap<String, String> = HashMap::new();

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let mut input = input.trim().to_string();

        let command = match parse_input(&mut input) {
            Ok(cmd) => cmd,
            Err(err) => {
                println!("Error: {err}");
                continue;
            }
        };

        match command {
            Command::Exit => {
                println!("EXIT command received, shutting down...");
                break;
            }
            Command::Get(key) => {
                let value = get(&store, key);

                println!("GET SUCCESS: {value}");

                continue;
            }
            Command::Set(key, value) => {
                set(&mut store, key, value);

                println!("SET SUCCESS");

                continue;
            }
        }
    }
}
