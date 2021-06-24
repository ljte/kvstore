use std::fmt;
use std::{env, process::exit};

#[derive(Debug, PartialEq)]
pub enum KVAction {
    SET,
    GET,
}

#[derive(Debug, PartialEq)]
pub struct UnknownAction<'a>(&'a str);

impl fmt::Display for UnknownAction<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unknown action {}", self.0)
    }
}

impl KVAction {
    pub fn from_str(s: &str) -> Result<Self, UnknownAction> {
        match s {
            "get" => Ok(Self::GET),
            "set" => Ok(Self::SET),
            _ => Err(UnknownAction(s)),
        }
    }
}

pub struct Args {
    pub action: KVAction,
    pub key: String,
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub struct ArgsParseError<'a>(&'a str);

impl Args {
    pub fn parse_args() -> Result<Self, ArgsParseError<'_>> {
        let mut args = env::args().skip(1);

        if args.len() < 3 {
            return Err(ArgsParseError("Not enough arguments"));
        }

        let action = KVAction::from_str(args.next().unwrap().as_ref()).unwrap_or_else(|err| {
            eprintln!("Error parsing args: {}", err);
            exit(1);
        });

        let key = args.next().unwrap();
        let value = args.next().unwrap();

        Ok(Self { action, key, value })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kvaction_parsed_from_str() {
        assert_eq!(KVAction::from_str("get"), Ok(KVAction::GET));
        assert_eq!(KVAction::from_str("set"), Ok(KVAction::SET));
    }

    #[test]
    fn test_kvaction_not_parsed_from_random_str() {
        todo!();
    }
}
