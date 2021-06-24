use kvstore::KVAction;

fn main() {
    let (action, key, value) = kvstore::parse_args();
    match action {
        KVAction::GET => {
            todo!();
        }
        KVAction::SET => { todo!();
        }
    }
}
