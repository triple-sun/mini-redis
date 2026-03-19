use std::{
    env,
    io::{self},
    sync::Arc,
};

use crate::{
    command::{Command, parse_input},
    options::{Mode, parse_options},
    storage::{Storage, log_store::LogStorage, mem_store::MemStorage},
};

mod command;
mod options;
mod storage;

fn main() {
    println!("Welcome to mini-redis!\n");

    let options = parse_options(env::args());

    options.print();

    let store: Arc<dyn Storage> = match options.mode {
        Mode::MemOnly => MemStorage::init(options),
        Mode::Default => LogStorage::init(options),
    };

    println!("");
    println!("Please input your command");
    println!("Type 'help' to list existing commands");

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
            Command::Help => {
                println!(
                    "Supported commands:\n'SET'/'set' {{key}} {{value}} - set value for key;\n'GET'/'get' {{key}} - get value for key\n'EXIT'/'exit' - exit the REPL loop;\n"
                );
                continue;
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

                println!("SET SUCCESS");

                continue;
            }
        }
    }
}
