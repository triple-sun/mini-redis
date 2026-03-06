use std::fmt::{self};

use crate::errors::DBError;

#[derive(Debug, PartialEq)]
pub enum Command<'a> {
    Get(&'a str),
    Set(&'a str, &'a str),
    Exit,
}

impl fmt::Display for Command<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::Exit => write!(f, "EXIT"),
            Command::Get(k) => write!(f, "GET {k}"),
            Command::Set(k, v) => write!(f, "SET {k} {v}"),
        }
    }
}

pub fn parse_input(input: &'_ String) -> Result<Command<'_>, DBError> {
    let input: Vec<&str> = input.split_whitespace().collect();
    let cmd = match input.get(0) {
        Some(&input_cmd) => input_cmd.to_uppercase(),
        None => return Err(DBError::EmptyInput),
    };

    match cmd.as_str() {
        "GET" | "SET" => (),
        "EXIT" => return Ok(Command::Exit),
        _ => return Err(DBError::UnexpectedCommand(cmd.to_string())),
    }

    let key = match input.get(1) {
        Some(&key) => key,
        None => return Err(DBError::KeyNotFound),
    };

    if cmd == "GET" {
        return Ok(Command::Get(key));
    }

    let value = match input.get(2) {
        Some(&value) => value,
        None => return Err(DBError::ValueNotFound),
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
            DBError::UnexpectedCommand(input.to_uppercase())
        );
    }

    #[test]
    fn handles_no_key() {
        let input = "get".to_string();
        let cmd = parse_input(&input);

        assert_eq!(cmd.err().unwrap(), DBError::KeyNotFound);
    }

    #[test]
    fn handles_no_value() {
        let input = "set key".to_string();
        let cmd = parse_input(&input);

        assert_eq!(cmd.err().unwrap(), DBError::ValueNotFound);
    }
}
