mod errors;
use errors::{ArgsParseError, UnknownAction};

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
pub struct Args {
    pub action: KVAction,
    pub key: String,
    pub value: String,
}

impl Args {
    pub fn parse_args<T: Iterator<Item = String>>(args: T) -> Result<Self, ArgsParseError> {
        let mut args = args.skip(1);

        let mut _parse_arg = |err_msg| match args.next() {
            Some(v) => Ok(v),
            None => Err(ArgsParseError(String::from(err_msg))),
        };

        let action = match KVAction::from_str(&_parse_arg("Missing action")?) {
            Ok(v) => v,
            Err(e) => {
                return Err(ArgsParseError(format!("{}", e)));
            }
        };
        let key = _parse_arg("Missing key")?;
        let value = _parse_arg("Missing value")?;

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
        assert_eq!(KVAction::from_str("GGEG"), Err(UnknownAction("GGEG")));
    }

    #[test]
    fn test_parse_args_object_created() {
        let args = Args::parse_args(
            ["get".to_string(), "key".to_string(), "value".to_string()].into_iter(),
        );
        assert_eq!(
            args,
            Ok(Args {
                action: KVAction::GET,
                key: "key".to_string(),
                value: "value".to_string(),
            })
        );
    }
}
