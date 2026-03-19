use std::{
    error::Error,
    fmt::{self},
};

#[derive(Debug, PartialEq)]
pub enum Command<'a> {
    Get(&'a str),
    Set(&'a str, &'a str),
    Exit,
    Help,
}

impl fmt::Display for Command<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::Exit => write!(f, "EXIT"),
            Command::Get(k) => write!(f, "GET {k}"),
            Command::Set(k, v) => write!(f, "SET {k} {v}"),
            Command::Help => write!(f, "HELP"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum CommandError {
    EmptyInput,
    KeyNotFound,
    ValueNotFound,
    UnexpectedCommand(String),
}

impl Error for CommandError {}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            CommandError::EmptyInput => write!(f, "Empty input received!"),
            CommandError::KeyNotFound => write!(f, "Key not found!"),
            CommandError::ValueNotFound => write!(f, "Value not found!"),
            CommandError::UnexpectedCommand(cmd) => write!(f, "Unexpected command: {cmd}"),
        }
    }
}

pub fn parse_input(input: &'_ String) -> Result<Command<'_>, CommandError> {
    let mut input = input.split_whitespace();

    let cmd = match input.next() {
        Some(cmd) => cmd.to_uppercase(),
        None => return Err(CommandError::EmptyInput),
    };

    match cmd.as_str() {
        "GET" | "SET" => (),
        "EXIT" => return Ok(Command::Exit),
        "HELP" => return Ok(Command::Help),
        _ => return Err(CommandError::UnexpectedCommand(cmd.to_string())),
    }

    let key = match input.next() {
        Some(key) => key,
        None => return Err(CommandError::KeyNotFound),
    };

    if cmd == "GET" {
        return Ok(Command::Get(key));
    }

    let value = match input.next() {
        Some(value) => value,
        None => return Err(CommandError::ValueNotFound),
    };

    Ok(Command::Set(key, value))
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn parses_exit() {
        let input = "exit".to_string();
        let cmd = parse_input(&input);

        assert_eq!(cmd.unwrap(), Command::Exit);
    }

    #[test]
    fn parses_get() {
        let input = "get key".to_string();
        let cmd = parse_input(&input);

        assert_eq!(cmd.unwrap(), Command::Get("key"));
    }

    #[test]
    fn parses_set() {
        let input = "set key value".to_string();
        let cmd = parse_input(&input);

        assert_eq!(cmd.unwrap(), Command::Set("key", "value"));
    }

    #[test]
    fn handles_unexpected_cmd() {
        let input = "test".to_string();
        let cmd = parse_input(&input);

        assert_eq!(
            cmd.err().unwrap(),
            CommandError::UnexpectedCommand(input.to_uppercase())
        );
    }

    #[test]
    fn handles_no_key() {
        let input = "get".to_string();
        let cmd = parse_input(&input);

        assert_eq!(cmd.err().unwrap(), CommandError::KeyNotFound);
    }

    #[test]
    fn handles_no_value() {
        let input = "set key".to_string();
        let cmd = parse_input(&input);

        assert_eq!(cmd.err().unwrap(), CommandError::ValueNotFound);
    }
}
