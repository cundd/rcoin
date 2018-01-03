use std::io::Write;
use std::io::stdout;
use super::MediumTrait;
use super::super::Error;

#[derive(Debug)]
pub struct Terminal {}

impl Terminal {
    pub fn new() -> Self {
        Terminal {}
    }
}

impl MediumTrait for Terminal {
    fn draw(&self, output: &str) -> Result<(), Error> {
        println!();
        print!("{}[2J", 27 as char); // Clear the screen

        print!("{}", output);

        Ok(())
    }

    fn flush(&self) -> Result<(), Error> {
        match stdout().flush() {
            Ok(_) => Ok(()),
            Err(e) => Err(ui_error!(OutputError, "{}", e.to_string())),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn print_test() {
        assert!(Terminal::new().draw("hello").is_ok());
        assert!(Terminal::new().draw("").is_ok());
    }
}
