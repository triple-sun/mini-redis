use std::fmt;

pub enum DBError {
    EmptyInput,
    KeyNotFound,
    ValueNotFound,
    UnexpectedCommand(String),
}

impl fmt::Display for DBError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            DBError::EmptyInput => write!(f, "Empty input received!"),
            DBError::KeyNotFound => write!(f, "Key not found!"),
            DBError::ValueNotFound => write!(f, "Value not found!"),
            DBError::UnexpectedCommand(cmd) => write!(f, "Unexpected command: {cmd}"),
        }
    }
}
