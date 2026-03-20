use std::{
    env,
    io::{self},
    sync::Arc,
};

use crate::{
    command::Command,
    config::{Config, Mode},
    storage::{Storage, log_store::LogStorage, mem_store::MemStorage},
    utils::print_err,
};

use colored::Colorize;

mod command;
mod config;
mod storage;
mod utils;

fn main() {
    println!("{}", "\nWELCOME TO MINI-REDIS!\n".blue().bold());

    let config = Config::from_flags(env::args());

    let store: Arc<dyn Storage> = match config.mode {
        Mode::MemOnly => MemStorage::init(config),
        Mode::Default => LogStorage::init(config),
    };

    println!("");
    println!("{}", "Please input your command".green().bold());
    println!("{}", "Type 'help' to list existing commands".green());

    loop {
        let mut input = String::new();

        if let Err(msg) = io::stdin().read_line(&mut input) {
            print_err(format!("Failed to read line: {msg}"));
        }

        let mut input = input.trim().to_string();

        let command = match Command::from(&mut input) {
            Ok(cmd) => cmd,
            Err(err) => {
                print_err(format!("Error: {err}"));
                continue;
            }
        };

        match command {
            Command::Exit => {
                print_err(
                    "EXIT command received, shutting down..."
                        .red()
                        .bold()
                        .to_string(),
                );
                break;
            }
            Command::Help => {
                println!(
                    "{}",
                    "Supported commands:\n'SET'/'set' {{key}} {{value}} - set value for key;\n'GET'/'get' {{key}} - get value for key\n'EXIT'/'exit' - exit the REPL loop;".on_blue()
                );
                continue;
            }
            Command::Get(key) => {
                match store.get(key) {
                    Ok(value) => println!("{}", value.green()),
                    Err(err) => print_err(format!("Could not get value from store: {err}")),
                }

                continue;
            }
            Command::Set(k, v) => {
                store.set(k, v);

                println!("{}", "SET SUCCESS".green());

                continue;
            }
        }
    }
}
