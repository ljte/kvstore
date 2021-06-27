use crate::errors::ArgsParseError;
use crate::storage::KVAction;

#[derive(Debug, PartialEq)]
pub struct Args {
    pub action: KVAction,
    pub key: String,
    pub value: Option<String>,
}

impl Args {
    pub fn parse_args<T>(args: T) -> Result<Self, ArgsParseError>
    where
        T: ExactSizeIterator<Item = String>,
    {
        let mut args = args;

        let mut parse_arg = |err_msg: &str| match args.next() {
            Some(v) => Ok(v),
            None => Err(ArgsParseError(err_msg.to_string())),
        };

        let action = match KVAction::from_str(&parse_arg("Missing action")?) {
            Ok(v) => v,
            Err(e) => {
                return Err(ArgsParseError(e.to_string()));
            }
        };
        let key = parse_arg("Missing key")?;
        let res_value = parse_arg("Missing value");
        let value = match action {
            KVAction::SET => Some(res_value?),
            KVAction::GET => {
                if let Ok(_) = res_value {
                    return Err(ArgsParseError(String::from("Too many arguments to get")));
                }
                None
            }
        };

        Ok(Self { action, key, value })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_args_object_created() {
        let strs = vec![String::from("get"), String::from("key")].into_iter();
        let args = Args::parse_args(strs);
        assert_eq!(
            args,
            Ok(Args {
                action: KVAction::GET,
                key: "key".to_string(),
                value: None,
            })
        );

        let strs = vec![
            String::from("set"),
            String::from("key"),
            String::from("value"),
        ]
        .into_iter();
        let args = Args::parse_args(strs);
        assert_eq!(
            args,
            Ok(Args {
                action: KVAction::SET,
                key: "key".to_string(),
                value: Some(String::from("value")),
            })
        );
    }

    #[test]
    fn test_parse_args_invalid_action() {
        let strs = vec![String::from("asdgsa")].into_iter();
        let args = Args::parse_args(strs);
        assert_eq!(
            args,
            Err(ArgsParseError(String::from("Unknown action: asdgsa")))
        );
    }

    #[test]
    fn test_parse_args_missing_key() {
        let strs = vec![String::from("set")].into_iter();
        let args = Args::parse_args(strs);
        assert_eq!(args, Err(ArgsParseError(String::from("Missing key"))));
    }

    #[test]
    fn test_parse_args_missing_value() {
        let strs = vec![String::from("set"), String::from("key")].into_iter();
        let args = Args::parse_args(strs);
        assert_eq!(args, Err(ArgsParseError(String::from("Missing value"))));
    }

    #[test]
    fn test_get_missing_key() {
        let strs = vec![String::from("get")].into_iter();
        let args = Args::parse_args(strs);
        assert_eq!(args, Err(ArgsParseError(String::from("Missing key"))));
    }

    #[test]
    fn test_get_too_many_args() {
        let strs = vec![
            String::from("get"),
            String::from("key"),
            String::from("val"),
        ]
        .into_iter();
        let args = Args::parse_args(strs);
        assert_eq!(
            args,
            Err(ArgsParseError(String::from("Too many arguments to get")))
        );
    }
}
