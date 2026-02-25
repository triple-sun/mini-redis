use crate::errors::DBError;

pub enum Command<'a> {
    Get(&'a str),
    Set(&'a str, &'a str),
    Exit,
}

pub fn parse_input<'a>(input: &'a mut str) -> Result<Command<'a>, DBError> {
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
