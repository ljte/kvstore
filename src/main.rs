use std::collections::HashMap;
use std::{env, process::exit};

enum KVAction {
    SET,
    GET,
}

struct UnknownAction;

impl From<&str> for KVAction {
    fn from(s: &str) -> Result<Self, UnknownAction> {
        match s.to_lowercase().as_ref() {
            "get" => Ok(Self::GET),
            "set" => Ok(Self::SET),
            _ => Err(UnknownAction),
        }
    }
}

fn parse_args() -> (String, String) {
    let mut args = env::args().skip(1);

    if args.len() < 3 {
        eprintln!("Not enough arguments");
        exit(1);
    }

    let action = 
    let key = args.next().unwrap();
    let value = args.next().unwrap();

    (key, value)
}

fn main() {
    let (key, value) = parse_args();
    let mut kvstore = HashMap::kvstore.insert(key, value);
    println!("{:?}", kvstore);
}
