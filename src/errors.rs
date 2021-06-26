use std::fmt;

#[derive(Debug, PartialEq)]
pub struct UnknownAction<'a>(pub &'a str);

impl fmt::Display for UnknownAction<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unknown action: {}", self.0)
    }
}

#[derive(Debug, PartialEq)]
pub struct ArgsParseError(pub String);

impl fmt::Display for ArgsParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error parsing args: {}", self.0)
    }
}
