use colored::Colorize;

pub fn prepare_key(ignore_case: bool, k: &str) -> String {
    if ignore_case {
        k.to_uppercase()
    } else {
        k.to_string()
    }
}

pub fn print_err(err: String) {
    eprintln!("{}", err.red())
}
