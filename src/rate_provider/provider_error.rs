use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ProviderError {
    message: String,
}

impl ProviderError {
    pub fn new(message: String) -> Self {
        ProviderError { message }
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