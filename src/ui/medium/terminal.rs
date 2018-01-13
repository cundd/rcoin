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
        let mut buffer = String::with_capacity(output.len() + 6);
        // Clear the screen
//        print!("{}[2J", 27 as char);

        // Set cursor to 0x0
        buffer.push(27 as char);
        buffer.push_str("[0;0H");

        buffer.push_str(output);
        buffer.shrink_to_fit();

        print!("{}", buffer);

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
