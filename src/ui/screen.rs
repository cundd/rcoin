use std::fmt;
use std::fmt::Debug;
use matrix::PointTrait;
use super::screen_buffer::ScreenBuffer;
use super::pixel::CoordinatePrecision;
use super::pixel::Pixel;
use super::pixel_sequence::*;
use super::style::*;
use super::medium;
use super::medium::MediumTrait;
use super::size::Size;
use super::error::Error;

pub const DEFAULT_WIDTH: CoordinatePrecision = 30;
pub const DEFAULT_HEIGHT: CoordinatePrecision = 10;

#[derive(Debug)]
pub struct Screen<T: MediumTrait + Debug> {
    buffer: ScreenBuffer,
    medium: T,
}

#[allow(unused)]
impl<T: MediumTrait + Debug> Screen<T> {
    pub fn new(size: Size, fill_pixel: Pixel, medium: T) -> Result<Self, Error> {
        let buffer = ScreenBuffer::new(size, fill_pixel)?;
        Ok(Screen { buffer, medium })
    }

    pub fn size(&self) -> Size {
        self.buffer.size()
    }

    /// Insert the text at the given point
    ///
    /// An error will be returned if the text contains a newline ('\n'), one of the text's
    /// characters does not fit into one row of the underlying buffer
    ///
    /// Keep in mind, that these operations are not transactional. If an error occurs with the nth
    /// character, the previous characters are still stored
    pub fn draw_text<P: PointTrait + Debug>(&mut self, point: &P, text: &str) -> Result<(), Error> {
        if let Some(_) = text.find('\n') {
            return Err(ui_error!(InputTextError, "Newline character must not appear in `draw_text()`"));
        }

        let mut current_x = point.x();
        let mut index = 0;
        let mut chars: Vec<char> = text.chars().collect();
        let mut styles = Styles::normal();

        while let Some(character) = chars.get(index) {
            match *character {
                '\n' => {}
                character @ _ if character.is_control() => {
                    let (offset, new_style) = Self::consume_control_sequence(text, index);
                    styles = new_style;
                    index += offset;
                }
                character @ _ => {
                    let pixel = Pixel::with_point_and_styles(character, &point.with_x(current_x), styles);
                    self.buffer.draw_pixel(pixel)?
                }
            }

            current_x += 1;
            index += 1;
        }

        Ok(())
    }

    /// Insert the text at the given point with support for multi-line text
    ///
    /// An error will be returned if one the text's characters does not fit into the underlying buffer
    ///
    /// Keep in mind, that these operations are not transactional. If an error occurs with the nth
    /// character, the previous characters are still stored
    pub fn draw_text_wrapping<P: PointTrait + Debug>(&mut self, point: &P, text: &str) -> Result<(), Error> {
        self.draw_multi_line_text_wrapping(point, text, true)
    }

    /// Insert the text at the given point with support for multi-line text
    ///
    /// An error will be returned if one the text's characters does not fit into the underlying buffer
    ///
    /// Keep in mind, that these operations are not transactional. If an error occurs with the nth
    /// character, the previous characters are still stored
    pub fn draw_multi_line_text<P: PointTrait + Debug>(&mut self, point: &P, text: &str) -> Result<(), Error> {
        self.draw_multi_line_text_wrapping(point, text, false)
    }

    /// Insert the text at the given point with support for multi-line text
   ///
   /// An error will be returned if one the text's characters does not fit into the underlying buffer
   ///
   /// Keep in mind, that these operations are not transactional. If an error occurs with the nth
   /// character, the previous characters are still stored
    fn draw_multi_line_text_wrapping<P: PointTrait + Debug>(&mut self, point: &P, text: &str, auto_wrap: bool) -> Result<(), Error> {
        let lines_count = text.matches('\n').count();
        if lines_count > self.buffer.height() as usize {
            return Err(ui_error!(SizeError, "Buffer height is {} but the input text contains {} lines", self.buffer.height(), lines_count));
        }

        let mut current_x = point.x();
        let mut current_y = point.y();
        let mut index = 0;
        let mut chars = PixelSequence::from_str(text);
        let mut style = Styles::normal();

        while let Some(pixel) = chars.get(index) {
            if auto_wrap {
                // If the maximum x for this line is reached, set the coordinates to a new row
                if current_x >= self.buffer.width() {
                    current_y += 1;
                    current_x = 0;

                    if pixel.character == '\n' {
                        index += 1;
                        continue;
                    }
                }
            }

            if pixel.character == '\n' {
                current_y += 1;
                current_x = 0;
                index += 1;

                continue;
            }
            self.buffer.draw_pixel(pixel.with_x_y(current_x, current_y))?;
            current_x += 1;
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

    #[inline]
    fn consume_control_sequence<'a>(text: &str, start_index: usize) -> (usize, Styles) {
        let mut offset = 0;
        let mut style_string = String::new();

        while let Some(character) = text.chars().nth(start_index + offset) {
            match character {
                'm' => break,
                '0' ... '9' => style_string.push(character),
                _ => {}
            };
            offset += 1;
        }

//        StyleTrait::from_str()
        let style = Styles::from_str(&style_string).unwrap_or(Styles::normal());

        (offset, style)
    }
}

#[allow(unused)]
impl Screen<medium::Terminal> {
    pub fn default() -> Result<Self, Error> {
        let size = Size::auto()?;
        Self::new(size, Pixel::placeholder(0, 0), medium::default())
    }

    pub fn with_size(size: Size) -> Result<Self, Error> {
        Self::new(size, Pixel::placeholder(0, 0), medium::default())
    }

    pub fn with_size_and_fill_pixel(size: Size, fill_pixel: Pixel) -> Result<Self, Error> {
        Self::new(size, fill_pixel, medium::default())
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
    fn get_contents_test() {
        let screen = Screen::with_size(Size::new(10, 5)).unwrap();
        assert_eq!("          \n          \n          \n          \n          \n", screen.get_contents());

        let screen = Screen::with_size_and_fill_pixel(Size::new(10, 5), Pixel::placeholder_with_character(0, 0, '.')).unwrap();
        assert_eq!(r"..........
..........
..........
..........
..........
", screen.get_contents());
    }

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