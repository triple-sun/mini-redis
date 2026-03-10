use std::{
    error::Error,
    fmt::{self},
};

#[derive(Debug, PartialEq)]
pub enum CommandError {
    EmptyInput,
    KeyNotFound,
    ValueNotFound,
    UnexpectedCommand(String),
}

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

impl Error for CommandError {}
