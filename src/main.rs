use kvstore::{Args, KVAction, KVStorage};
use std::env;

fn main() {
    let args = match Args::parse_args(env::args().skip(1)) {
        Ok(args) => args,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    let storage = KVStorage::new("db");

    match args.action {
        KVAction::GET => {
            let val = match storage.get(&args.key) {
                Some(value) => value,
                None => format!("key {} is not set", args.key),
            };
            println!("{}", val);
        }
        KVAction::SET => {
            let value = args.value.unwrap();
            if let Err(e) = storage.set(&args.key, &value) {
                eprintln!("Failed to set {}: {}", args.key, e);
                std::process::exit(1);
            }
        }
    };
}
