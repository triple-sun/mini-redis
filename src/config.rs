use colored::Colorize;
use core::fmt;
use std::{cmp, fs::OpenOptions, io::Error, num::ParseIntError};

use crate::utils::print_err;

const DEFAULT_COMPACT_INTERVAL: u64 = 15;
const DEFAULT_LOG_FILE_PATH: &str = "./db_log/db.log";

#[derive(Debug, PartialEq)]
pub enum Mode {
    MemOnly,
    Default,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::MemOnly => write!(f, "Memory only storage"),
            _ => write!(f, "Default (Log storage)"),
        }
    }
}

#[derive(Debug)]
pub enum OptionError {
    IntervalMissing,
    IntervalIncorrect(String, ParseIntError),
    ModeMissing,
    ModeIncorrect(String),
    LogFilePathMissing,
    LogFilePathIncorrect(String, Error),
    UnexpectedFlag(String),
}

impl fmt::Display for OptionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            OptionError::IntervalMissing => write!(
                f,
                "--compact_interval value is missing, using default: {DEFAULT_COMPACT_INTERVAL}"
            ),
            OptionError::IntervalIncorrect(value, err) => write!(
                f,
                "--compact_interval value '{value}' is incorrect: {err}, using default: {DEFAULT_COMPACT_INTERVAL}"
            ),
            OptionError::ModeMissing => write!(f, "--mode value is missing, using default"),
            OptionError::ModeIncorrect(value) => {
                write!(f, "--mode value '{value}' is incorrect, using default")
            }
            OptionError::LogFilePathMissing => write!(
                f,
                "--log_file_path value is missing, using default: {DEFAULT_LOG_FILE_PATH}"
            ),
            OptionError::LogFilePathIncorrect(value, err) => {
                write!(f, "--log_file_path value {value} is incorrect: {err}")
            }
            OptionError::UnexpectedFlag(flag) => write!(f, "Unexpected flag: {flag}"),
        }
    }
}

pub struct Config {
    pub mode: Mode,
    pub ignore_case: bool,
    pub compact_interval: u64,
    pub log_file_path: String,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let fields = vec![
            ("Mode", self.mode.to_string()),
            ("Ignore case", self.ignore_case.to_string()),
            ("Compact interval", self.compact_interval.to_string()),
            ("Log file path", self.log_file_path.to_string()),
        ];

        for (field, value) in fields.iter() {
            writeln!(
                f,
                "{}: {}",
                format!("{field}").blue().bold(),
                format!("{value}").yellow()
            )?;
        }

        Ok(())
    }
}

impl Config {
    pub fn print(&self) {
        print!("{}", self)
    }

    pub fn from_flags(args: impl Iterator<Item = String> ) -> Config {
        let mut config = Config {
        mode: Mode::Default,
        ignore_case: false,
        compact_interval: 15,
        log_file_path: DEFAULT_LOG_FILE_PATH.to_string(),
    };

    let mut errors: Vec<OptionError> = vec![];

    for mut arg in args {
        if arg.starts_with("--") {
            let arg = arg.split_off(2);

            let arg: Vec<&str> = arg.split('=').collect();

            let flag = match arg.get(0) {
                Some(flag) => flag,
                None => continue,
            };

            match *flag {
                "mode" => {
                    match arg.get(1) {
                        Some(value) => match *value {
                            "mem_only" => config.mode = Mode::MemOnly,
                            "default" => config.mode = Mode::Default,
                            _else => errors.push(OptionError::ModeIncorrect(_else.to_string())),
                        },
                        None => {
                            errors.push(OptionError::ModeMissing);
                            continue;
                        }
                    };
                }
                "compact_interval" => match arg.get(1) {
                    Some(value) => match value.parse::<u64>() {
                        Ok(value) => config.compact_interval = cmp::max(0, value),
                        Err(err) => {
                            errors.push(OptionError::IntervalIncorrect(value.to_string(), err))
                        }
                    },
                    None => {
                        errors.push(OptionError::IntervalMissing);
                        continue;
                    }
                },
                "log_file_path" => match arg.get(1) {
                    Some(value) => match OpenOptions::new()
                        .read(true)
                        .append(true)
                        .create(true)
                        .open(value)
                    {
                        Ok(_) => config.log_file_path = value.to_string(),
                        Err(err) => {
                            errors.push(OptionError::LogFilePathIncorrect(value.to_string(), err))
                        }
                    },
                    None => {
                        errors.push(OptionError::LogFilePathMissing);
                        continue;
                    }
                },
                "ignore_case" => config.ignore_case = true,
                _else => errors.push(OptionError::UnexpectedFlag(_else.to_string())),
            }
        }
    }

    config.print();

    if errors.len() > 0 {
        let errors = errors
            .iter()
            .map(|e| format!("{e}"))
            .collect::<Vec<String>>()
            .join(", ");

        print_err(format!("Errors: {errors:#?}\n"));
    }

    config
    }
}


#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn parses_no_flags() {
        let args: Vec<String> = vec![];

        let flags = Config::from_flags(args.into_iter());

        assert_eq!(flags.mode, Mode::Default);
        assert_eq!(flags.compact_interval, 15);
        assert_eq!(flags.ignore_case, false);
    }

    #[test]
    fn parses_correct_flags() -> Result<(), Box<dyn std::error::Error>> {
        let args: Vec<String> = vec![
            String::from("--mode=mem_only"),
            String::from("--compact_interval=200"),
            String::from("--ignore_case"),
            String::from("--log_file_path=./log.db"),
        ];

        let flags = Config::from_flags(args.into_iter());

        assert_eq!(flags.mode, Mode::MemOnly);
        assert_eq!(flags.ignore_case, true);
        assert_eq!(flags.compact_interval, 200);
        assert_eq!(flags.log_file_path, "./log.db");

        fs::remove_file("./log.db")?;
        Ok(())
    }

    #[test]
    fn ignores_incorrect_flags() {
        let args: Vec<String> = vec![
            String::from("--mode=testmode"),
            String::from("--compact_interval=abc"),
            String::from("--ignore_case=test"),
            String::from("--log_file_path=.../1223456"),
            String::from("--some_flag"),
            String::from("not a flag"),
        ];

        let flags = Config::from_flags(args.into_iter());

        assert_eq!(flags.mode, Mode::Default);
        assert_eq!(flags.compact_interval, DEFAULT_COMPACT_INTERVAL);
        assert_eq!(flags.ignore_case, true);
        assert_eq!(flags.log_file_path, DEFAULT_LOG_FILE_PATH);
    }
}
