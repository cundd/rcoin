use std::fmt;


macro_rules! ui_error {
($type:ident,$($arg:tt)*) => ($crate::ui::error::Error::$type($crate::std::fmt::format(format_args!($($arg)*))));
}


#[derive(Debug, PartialOrd, PartialEq)]
pub enum Error {
    InputTextError(String),
    SizeError(String),
    OutputError(String),
}

//impl Error {
//    pub fn new<S>(message: S) -> Self
//        where S: Into<String> {
//        Self::x { message: message.into() }
//    }
//}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::InputTextError(ref message) => write!(f, "{}", message),
            &Error::SizeError(ref message) => write!(f, "{}", message),
            &Error::OutputError(ref message) => write!(f, "{}", message),
        }
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
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

//#[allow(unused)]
//macro_rules! new_err {
//    ($name:ident($($arg:tt)*)) => (
//
//        #[derive(Debug)]
//        struct $name {
//            message: String,
//        }
//
//        impl $name {
//            pub fn new<S>(message: S) -> Self where S: Into<String> {
//                $name { message: message.into() }
//            }
//        }
//
//        impl ::std::fmt::Display for $name {
//            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
//                write!(f, "{}", self.message)
//            }
//        }
//
//        impl ::std::error::Error for $name {
//            fn description(&self) -> &str {
//                &self.message
//            }
//        }
//    );
//