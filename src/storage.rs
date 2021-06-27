use crate::errors::UnknownAction;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};

#[derive(Debug, PartialEq)]
pub enum KVAction {
    SET,
    GET,
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

#[derive(Debug, PartialEq)]
pub struct KVStorage<'a> {
    filename: &'a str,
}

impl<'a> KVStorage<'a> {
    pub fn new(filename: &'a str) -> Self {
        Self { filename }
    }

    pub fn set(&self, key: &str, value: &str) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.filename)?;

        writeln!(&mut file, "{} {}", key, value)?;

        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let contents = match fs::read_to_string(self.filename) {
            Ok(v) => v,
            Err(_) => return None,
        };

        let matches = contents
            .lines()
            .filter(|line| line.split(' ').next().unwrap() == key)
            .next_back();

        let (_, value) = matches?.split_once(" ").unwrap();

        Some(String::from(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_kvaction_parsed_from_str() {
        assert_eq!(KVAction::from_str("get"), Ok(KVAction::GET));
        assert_eq!(KVAction::from_str("set"), Ok(KVAction::SET));
    }

    #[test]
    fn test_kvaction_not_parsed_from_random_str() {
        assert_eq!(KVAction::from_str("GGEG"), Err(UnknownAction("GGEG")));
    }

    #[test]
    fn test_storage_creation() {
        let s = KVStorage::new("TEST.txt");
        assert_eq!(
            s,
            KVStorage {
                filename: "TEST.txt"
            }
        );
    }

    #[test]
    fn test_set_wrote_to_file() {
        let s = KVStorage::new("test1");
        s.set("hey", "nice").unwrap();
        let contents = fs::read_to_string(s.filename).unwrap();

        assert_eq!(contents, "hey nice\n");

        fs::remove_file(s.filename).unwrap();
    }

    #[test]
    fn test_get_returns_value() {
        let s = KVStorage::new("test2");
        s.set("hey", "nice").unwrap();

        assert_eq!(s.get("hey"), Some(String::from("nice")));

        fs::remove_file(s.filename).unwrap();
    }

    #[test]
    fn test_get_returns_none_on_empty_storage() {
        let s = KVStorage::new("test2");
        assert_eq!(s.get("some"), None);
    }

    #[test]
    fn test_get_return_last_set_value() {
        let s = KVStorage::new("test3");
        s.set("hey", "nice").unwrap();
        s.set("hey", "test").unwrap();

        assert_eq!(s.get("hey"), Some(String::from("test")));

        fs::remove_file(s.filename).unwrap();
    }

    #[test]
    fn test_get_returns_none_on_unset_key() {
        let s = KVStorage::new("test4");
        s.set("hey", "nice").unwrap();

        assert_eq!(s.get("gsd"), None);

        fs::remove_file(s.filename).unwrap();
    }

    #[test]
    fn test_get_on_long_string() {
        let s = KVStorage::new("test5");
        s.set("hey", "{'hey': ['look', 'here'], 'what': 'sup'}")
            .unwrap();

        assert_eq!(
            s.get("hey"),
            Some(String::from("{'hey': ['look', 'here'], 'what': 'sup'}")),
        );

        fs::remove_file(s.filename).unwrap();
    }
}
