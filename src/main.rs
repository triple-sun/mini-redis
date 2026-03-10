use std::{
    io::{self},
    sync::Arc,
    thread::{self, sleep},
    time::Duration,
};

use crate::{
    command::{Command, parse_input},
    store::Store,
};

mod command;
mod errors;
mod store;

fn main() {
    let store = Arc::new(Store::init());

    println!(
        "Welcome to mini-redis!\nPlease input your command\nSupported commands:\n`EXIT`, `exit` to exit;"
    );

    let compact_store = Arc::clone(&store);
    thread::spawn(move || {
        loop {
            sleep(Duration::new(15, 0));
            if let Err(err) = compact_store.compact_log() {
                eprintln!("Could not compact store: {err}");
            }
        }
    });

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
                match store.get(key) {
                    Ok(value) => println!("{value}"),
                    Err(err) => eprintln!("Could not get value from store: {err}"),
                }

                continue;
            }
            Command::Set(k, v) => {
                store.set(k, v);

                if let Err(err) = store.save_to_log(k, v) {
                    eprintln!("Could not save to log: {err}");
                };

                println!("SET SUCCESS");

                continue;
            }
        }
    }
}
