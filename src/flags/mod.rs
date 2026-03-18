use core::fmt;

#[derive(Debug, PartialEq)]
pub enum Mode {
    MemOnly,
    Default,
}

#[derive(Debug, PartialEq)]
pub enum Flag {
    Mode(Mode),
}

impl fmt::Display for Flag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Flag::Mode(mode) => write!(f, "Mode: {mode:?}"),
        }
    }
}

pub fn parse_flags(args: impl Iterator<Item = String>) -> Vec<Flag> {
    let mut result: Vec<Flag> = vec![];

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
                    let value = match arg.get(1) {
                        Some(value) => value,
                        None => {
                            result.push(Flag::Mode(Mode::Default));
                            continue;
                        }
                    };

                    match *value {
                        "memory-only" | "mem-only" | "mem_only" | "mem" => {
                            result.push(Flag::Mode(Mode::MemOnly));
                        }
                        _ => result.push(Flag::Mode(Mode::Default)),
                    }
                }
                _ => (),
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_no_flags() {
        let args: Vec<String> = vec![];

        let flags = parse_flags(args.into_iter());

        assert_eq!(flags.len(), 0);
    }

    #[test]
    fn parses_default_mode() {
        let args: Vec<String> = vec![String::from("--mode=")];

        let flags = parse_flags(args.into_iter());

        assert_eq!(flags.len(), 1);
        assert!(flags.contains(&Flag::Mode(Mode::Default)))
    }

    #[test]
    fn parses_mem_only_mode() {
        let args: Vec<String> = vec![String::from("--mode=mem_only")];

        let flags = parse_flags(args.into_iter());

        assert_eq!(flags.len(), 1);
        assert!(flags.contains(&Flag::Mode(Mode::MemOnly)));
    }
}
