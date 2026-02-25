use std::io::{self};

enum Command<'a> {
    Get(&'a str),
    Set(&'a str, &'a str),
    Exit,
}

fn parse_input<'a>(input: &'a str) -> Result<Command<'a>, String> {
    let input: Vec<&str> = input.split_whitespace().collect();
    let cmd = match input.get(0) {
        Some(&input_cmd) => input_cmd,
        None => return Err("Empty input received!".to_string()),
    };

    match cmd {
        "GET" | "SET" => (),
        "EXIT" => return Ok(Command::Exit),
        _ => return Err(format!("Unexpected command: {cmd}")),
    }

    let key = match input.get(1) {
        Some(&key) => key,
        None => return Err("Key is missing from command!".to_string()),
    };

    if cmd == "GET" {
        return Ok(Command::Get(key));
    }

    let value = match input.get(2) {
        Some(&value) => value,
        None => return Err("Value is missing from SET command!".to_string()),
    };

    Ok(Command::Set(key, value))
}

fn main() {
    println!(
        "Welcome to mini-redis!\nPlease input your command\nSupported commands:\n`EXIT`, `exit` to exit;"
    );
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim().to_uppercase();

        let command = match parse_input(&input) {
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
                println!("GET command received! Key: {key}");
                continue;
            }
            Command::Set(key, value) => {
                println!("SET command received! Key: {key}, value: {value}");
                continue;
            }
        }
    }
}
