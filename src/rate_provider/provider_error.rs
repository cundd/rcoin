use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ProviderError {
    message: String,
}

impl ProviderError {
    pub fn new<S>(message: S) -> Self
        where S: Into<String> {
        ProviderError { message: message.into() }
    }
}


impl fmt::Display for ProviderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ProviderError {
    fn description(&self) -> &str {
        &self.message
    }
}