use kvstore::{Args, KVAction};
use std::env;

fn main() {
    let args = match Args::parse_args(env::args()) {
        Ok(args) => args,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    println!("{:?}", args);

    match args.action {
        KVAction::GET => {
            todo!();
        }
        KVAction::SET => {
            todo!();
        }
    };
}
