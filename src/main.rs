use std::io::{self};

use crate::{
    command::{Command, parse_input},
    store::Store,
};

mod command;
mod constants;
mod errors;
mod store;

fn main() {
    let mut store = Store::init();

    store.restore();

    println!(
        "Welcome to mini-redis!\nPlease input your command\nSupported commands:\n`EXIT`, `exit` to exit;"
    );

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
                let value = store.get(key);

                println!("{value}");

                continue;
            }
            Command::Set(k, v) => {
                store.set(k, v);
                store.save(k, v);

                println!("SET SUCCESS");

                continue;
            }
        }
    }
}
