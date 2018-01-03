pub mod terminal;

use super::Error;
pub use self::terminal::Terminal;

pub trait MediumTrait {
    /// Output the given data to the output medium
    fn draw(&self, output: &str) -> Result<(), Error>;

    /// Explicitly flush the output buffer
    fn flush(&self) -> Result<(), Error>;
}

pub fn default() -> Terminal {
    Terminal::new()
}
