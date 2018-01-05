use std::fmt;

macro_rules! ui_error {
($type:ident,$($arg:tt)*) => (self::Error::$type($crate::std::fmt::format(format_args!($($arg)*))));
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Error {
    Misc(String),
    InputTextError(String),
    SizeError(String),
    OutputError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::Misc(ref message) => write!(f, "{}", message),
            &Error::InputTextError(ref message) => write!(f, "{}", message),
            &Error::SizeError(ref message) => write!(f, "{}", message),
            &Error::OutputError(ref message) => write!(f, "{}", message),
        }
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            &Error::Misc(ref message) => message,
            &Error::InputTextError(ref message) => message,
            &Error::SizeError(ref message) => message,
            &Error::OutputError(ref message) => message,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ui_error_test() {
        assert_eq!(
            Error::SizeError("Some size error".to_string()),
            ui_error!(SizeError, "{}", "Some size error")
        );
    }
}
