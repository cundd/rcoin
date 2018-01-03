use term_size;
use std::fmt;
use std::fmt::Debug;
use matrix::PointTrait;
use super::screen_buffer::ScreenBuffer;
use super::medium;
use super::medium::MediumTrait;
use super::size::Size;
use super::error::Error;


fn get_screen_size() -> Option<Size> {
    match term_size::dimensions() {
        Some((width, height)) => Some(Size::new(width, height)),
        None => Some(Size::new(10, 10)),
    }
}

#[derive(Debug)]
pub struct Screen<T: MediumTrait + Debug> {
    buffer: ScreenBuffer,
    medium: T,
}


#[allow(unused)]
impl<T: MediumTrait + Debug> Screen<T> {
    pub fn new(size: Size, medium: T) -> Result<Self, Error> {
        let buffer = ScreenBuffer::with_size(size)?;
        Ok(Screen { buffer, medium })
    }

    pub fn draw_text<P: PointTrait + Debug>(&mut self, point: &P, text: &str) -> Result<(), Error> {
        let mut current_x = point.x();
        let mut index = 0;

        while let Some(character) = text.chars().nth(index) {
            match character {
                '\n' => {
                    return Err(ui_error!(InputTextError, "Newline character must not appear in `draw_text()`"));
                }
                character @ _ if character.is_control() => {
                    index += Self::consume_control_sequence(text, index)
                }
                character @ _ => {
                    self.buffer.draw_point(&point.with_x(current_x), character)?
                }
            }

            current_x += 1;
            index += 1;
        }

        Ok(())
    }

    pub fn draw_text_wrapping<P: PointTrait + Debug>(&mut self, point: &P, text: &str) -> Result<(), Error> {
        let mut current_x = point.x();
        let mut current_y = point.y();
        let mut index = 0;

        while let Some(character) = text.chars().nth(index) {
            // If the maximum x for this line is reached, set the coordinates to a new row
            if current_x >= self.buffer.width() {
                current_y += 1;
                current_x = 0;
            }

            match character {
                '\n' => {
                    current_y += 1;
                    current_x = 0;
                    index += 1;

                    continue;
                }
                character @ _ if character.is_control() => index += Self::consume_control_sequence(text, index),
                character @ _ => {
                    self.buffer.draw_point(&point.with_x_y(current_x, current_y), character)?;
                    current_x += 1;
                }
            }

            if index > 10_000 {
                panic!("Stop at run {} with character `{}`", index, character);
            }

            index += 1;
        }

        Ok(())
    }

    pub fn get_contents(&self) -> String {
        self.buffer.get_contents()
    }

    pub fn flush(&self) -> Result<(), Error> {
        self.medium.draw(&self.buffer.get_contents())
    }

    fn consume_control_sequence<'a>(text: &str, start_index: usize) -> usize {
        let mut offset = 0;

        while let Some(character) = text.chars().nth(start_index + offset) {
            if character == 'm' {
                break;
            }
            offset += 1;
        }

        offset
    }
}

#[allow(unused)]
impl Screen<medium::Terminal> {
    pub fn default() -> Result<Self, Error> {
        Self::new(get_screen_size().unwrap(), medium::default())
    }

    pub fn with_size(size: Size) -> Result<Self, Error> {
        Self::new(size, medium::default())
    }
}


impl<T: MediumTrait + Debug> fmt::Display for Screen<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_contents())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use point::Point;

    #[test]
    fn draw_text_test() {
        let mut screen = Screen::with_size(Size::new(10, 5)).unwrap();

        let result = screen.draw_text(&Point::new(1, 2), "hello");
        assert!(result.is_ok(), "{}", result.unwrap_err());
        assert_eq!("          \n          \n hello    \n          \n          \n", screen.get_contents());

        let mut screen = Screen::with_size(Size::new(10, 5)).unwrap();
        assert!(screen.draw_text(&Point::new(0, 0), "hello").is_ok());
        assert_eq!("hello     \n          \n          \n          \n          \n", screen.get_contents());

        let mut screen = Screen::with_size(Size::new(10, 5)).unwrap();
        assert!(screen.draw_text(&Point::new(1, 2), "Daniel").is_ok());
        assert!(screen.draw_text(&Point::new(0, 0), "hello").is_ok());
        assert_eq!("hello     \n          \n Daniel   \n          \n          \n", screen.get_contents());
    }

    #[test]
    fn draw_text_overwrite_test() {
        let mut screen = Screen::with_size(Size::new(10, 5)).unwrap();
        assert!(screen.draw_text(&Point::new(1, 2), "hello").is_ok());
        assert!(screen.draw_text(&Point::new(1, 2), "bonjour").is_ok());

        assert_eq!("          \n          \n bonjour  \n          \n          \n", screen.get_contents());
    }

    #[test]
    fn draw_text_overflow_test() {
        let mut screen = Screen::with_size(Size::new(10, 5)).unwrap();
        assert!(screen.draw_text(&Point::new(7, 2), "hello").is_err());
        assert_eq!(
            Err(Error::SizeError("The point's `x` coordinate (10) is bigger than the screen's width (10) for character 'l'".to_string())),
            screen.draw_text(&Point::new(7, 2), "hello")
        );
    }

    #[test]
    fn draw_text_wrapping_test() {
        let mut screen = Screen::with_size(Size::new(10, 5)).unwrap();
        assert!(screen.draw_text_wrapping(&Point::new(7, 2), "hello").is_ok());
        assert_eq!("          \n          \n       hel\nlo        \n          \n", screen.get_contents());
    }

    #[test]
    fn draw_text_wrapping_overflow_test() {
        let mut screen = Screen::with_size(Size::new(10, 3)).unwrap();
        assert!(screen.draw_text_wrapping(&Point::new(7, 2), "hello").is_err());
        assert_eq!(
            Err(Error::SizeError("The point's `y` coordinate (3) is bigger than the screen's height (3) for character 'l'".to_string())),
            screen.draw_text_wrapping(&Point::new(7, 2), "hello")
        );
    }
}