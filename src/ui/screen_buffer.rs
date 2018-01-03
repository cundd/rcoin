use matrix::PointTrait;
use super::size::Size;
use super::error::*;

#[derive(Debug)]
pub struct ScreenBuffer {
    size: Size,
    buffer: Vec<char>,
}

impl ScreenBuffer {
    pub fn with_size(size: Size) -> Result<ScreenBuffer, Error> {
        Self::check_size(&size)?;

        let buffer = vec![' '; size.width * size.height];

        Ok(ScreenBuffer { size, buffer })
    }

    pub fn width(&self) -> usize {
        self.size.width
    }

    pub fn height(&self) -> usize {
        self.size.height
    }

    pub fn draw_point<T: PointTrait>(&mut self, point: &T, content: char) -> Result<(), Error> {
        let index = match self.get_index_for_point(point) {
            Ok(i) => i,
            Err(e) => return Err(ui_error!(SizeError,"{} for character '{}'", e.to_string(), content))
        };

        self.buffer[index] = content;

        Ok(())
    }

    pub fn get_content_at_point<T: PointTrait>(&self, point: &T) -> Result<char, Error> {
        let index = self.get_index_for_point(point)?;

        unsafe {
            Ok(*self.buffer.get_unchecked(index))
        }

//        match self.buffer.get(index) {
//            Some(c) => Ok(*c),
//            None => Ok(' '),
//        }
    }

    /// @TODO: Make Cow
    pub fn get_contents(&self) -> String {
        let width = self.size.width;
        let height = self.size.height;
        let mut row = 0;
        let mut output = String::with_capacity(width * height + height);

        while row < height {
            let start_index = row * width;
            let end_index = start_index + width;
            let mut index = start_index;
            while index < end_index {
                output.push(*self.buffer.get(index).expect(&format!("Index {} for row {} is out of range", index, row)));
                index += 1;
            }
            output.push('\n');
            row += 1;
        }

        output
    }

    fn check_size(size: &Size) -> Result<(), Error> {
        if size.width == 0 || size.height == 0 {
            return Err(ui_error!(SizeError, "The screen size {}x{} is too small", size.width, size.height));
        }
        if let Some(pixel_count) = size.width.checked_mul(size.height) {
            if let Some(_) = pixel_count.checked_add(size.height) {
                return Ok(());
            }
        }
        Err(ui_error!(SizeError, "The screen size {}x{} is too big", size.width, size.height))
    }

    fn check_point_bounds<T: PointTrait>(&self, point: &T) -> Result<(), Error> {
        if self.size.width == 0 || point.x() > self.size.width - 1 {
            return Err(ui_error!(SizeError, "The point's `x` coordinate ({}) is bigger than the screen's width ({})", point.x(), self.size.width));
        }
        if self.size.height == 0 || point.y() > self.size.height - 1 {
            return Err(ui_error!(SizeError, "The point's `y` coordinate ({}) is bigger than the screen's height ({})", point.y(), self.size.height));
        }

        Ok(())
    }

    fn get_index_for_point<T: PointTrait>(&self, point: &T) -> Result<usize, Error> {
        self.check_point_bounds(point)?;

        Ok(point.y() * self.size.width + point.x())
    }

//    fn is_ascii_control(c: char) -> bool {
//        (c as u32 <= 0x7f) && Self::is_ascii_control_u8(c as u8)
//    }
//
//    #[inline]
//    fn is_ascii_control_u8(c: u8) -> bool {
//        if c >= 0x80 { return false; }
//        match ASCII_CHARACTER_CLASS[c as usize] {
//            C | Cw => true,
//            _ => false
//        }
//    }
}
//
//enum AsciiCharacterClass {
//    C,
//    // control
//    Cw,
//    // control whitespace
//    W,
//    // whitespace
//    D,
//    // digit
//    L,
//    // lowercase
//    Lx,
//    // lowercase hex digit
//    U,
//    // uppercase
//    Ux,
//    // uppercase hex digit
//    P,  // punctuation
//}
//
//use self::AsciiCharacterClass::*;
//
//static ASCII_CHARACTER_CLASS: [AsciiCharacterClass; 128] = [
////  _0 _1 _2 _3 _4 _5 _6 _7 _8 _9 _a _b _c _d _e _f
//    C, C, C, C, C, C, C, C, C, Cw, Cw, C, Cw, Cw, C, C, // 0_
//    C, C, C, C, C, C, C, C, C, C, C, C, C, C, C, C, // 1_
//    W, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, // 2_
//    D, D, D, D, D, D, D, D, D, D, P, P, P, P, P, P, // 3_
//    P, Ux, Ux, Ux, Ux, Ux, Ux, U, U, U, U, U, U, U, U, U, // 4_
//    U, U, U, U, U, U, U, U, U, U, U, P, P, P, P, P, // 5_
//    P, Lx, Lx, Lx, Lx, Lx, Lx, L, L, L, L, L, L, L, L, L, // 6_
//    L, L, L, L, L, L, L, L, L, L, L, P, P, P, P, C, // 7_
//];

#[cfg(test)]
mod tests {
    use matrix::PointTrait;
    use super::*;
    use point::Point;

    #[test]
    fn draw_point_should_fail_test() {
        assert_eq!(
            Err(Error::SizeError("The point's `x` coordinate (2000000) is bigger than the screen's width (1) for character ' '".to_string())),
            ScreenBuffer::with_size(Size::new(1, 10)).unwrap()
                .draw_point(&Point::new(2_000_000, 0), ' ')
        );
        assert_eq!(
            Err(Error::SizeError("The point's `y` coordinate (3000000) is bigger than the screen's height (1) for character ' '".to_string())),
            ScreenBuffer::with_size(Size::new(10, 1)).unwrap()
                .draw_point(&Point::new(0, 3_000_000), ' ')
        );
        assert_eq!(
            Err(Error::SizeError("The point's `x` coordinate (10) is bigger than the screen's width (10) for character ' '".to_string())),
            ScreenBuffer::with_size(Size::new(10, 10)).unwrap()
                .draw_point(&Point::new(10, 10), ' ')
        );
    }

    #[test]
    fn draw_point_test() {
        let mut buffer = ScreenBuffer::with_size(Size::new(3_200, 1_024)).unwrap();
        let point = Point::new(100, 200);

        let buffer_size_before_draw = buffer.buffer.len();
        assert!(buffer.draw_point(&point, 'x').is_ok());
        assert_eq!('x', buffer.get_content_at_point(&point).expect(&format!("Failed to fetch content at {}", point)));

        assert_eq!(buffer_size_before_draw, buffer.buffer.len());
    }

    #[test]
//    fn draw_point_with_control_character() {
//        // is_control
//        let mut buffer = ScreenBuffer::with_size(Size::new(10, 20)).unwrap();
//
//        println!(" \u{1b}[");
//        assert!(buffer.draw_point(&Point::new(5, 10), 'x').is_ok());
//        assert_eq!("          \n          \n          \n          \n          \n          \n          \n          \n          \n          \n     x    \n          \n          \n          \n          \n          \n          \n          \n          \n          \n", buffer.get_contents());
//
//        //assert_eq!("\u{1b}[41mmy message\u{1b}[49m", bg_red("my message"));
//        //assert_eq!("\u{1b}[41mmy message\u{1b}[0m", style("my message", BgRed));
//        let mut buffer = ScreenBuffer::with_size(Size::new(10, 2)).unwrap();
//
//        assert!(buffer.draw_point(&Point::new(1, 0), '\u{1b}').is_ok());
//        assert_eq!(" \u{1b}         \n          \n", buffer.get_contents());
//
//        assert!(buffer.draw_point(&Point::new(2, 0), '[').is_ok());
//        assert!(buffer.draw_point(&Point::new(3, 0), '4').is_ok());
//        assert!(buffer.draw_point(&Point::new(4, 0), '1').is_ok());
//        assert!(buffer.draw_point(&Point::new(5, 0), 'm').is_ok());
//        assert_eq!(" \u{1b}[41m         \n          \n", buffer.get_contents());
//    }

    #[test]
    fn get_content_at_point_test() {
        let buffer = ScreenBuffer::with_size(Size::new(3_200, 1_024)).unwrap();

        assert_eq!(
            Err(Error::SizeError("The point's `x` coordinate (10000) is bigger than the screen's width (3200)".to_string())),
            buffer.get_content_at_point(&Point::new(10_000, 200))
        );
        assert_eq!(
            Err(Error::SizeError("The point's `x` coordinate (3200) is bigger than the screen's width (3200)".to_string())),
            buffer.get_content_at_point(&Point::new(3_200, 1_000))
        );

        assert_eq!(
            Err(Error::SizeError("The point's `y` coordinate (1024) is bigger than the screen's height (1024)".to_string())),
            buffer.get_content_at_point(&Point::new(3_000, 1_024))
        );

        assert_eq!(
            ' ',
            buffer.get_content_at_point(&Point::new(10, 200)).unwrap()
        );
    }

    #[test]
    fn get_contents_empty_test() {
        assert_eq!("  \n", ScreenBuffer::with_size(Size::new(2, 1)).unwrap().get_contents());
        assert_eq!("  \n  \n", ScreenBuffer::with_size(Size::new(2, 2)).unwrap().get_contents());
    }

    #[test]
    fn get_contents_test() {
        let mut buffer = ScreenBuffer::with_size(Size::new(2, 2)).unwrap();
        assert!(buffer.draw_point(&Point::new(0, 0), 'x').is_ok());
        assert_eq!("x \n  \n", buffer.get_contents());

        let mut buffer = ScreenBuffer::with_size(Size::new(10, 20)).unwrap();
        assert!(buffer.draw_point(&Point::new(5, 10), 'x').is_ok());
        assert_eq!("          \n          \n          \n          \n          \n          \n          \n          \n          \n          \n     x    \n          \n          \n          \n          \n          \n          \n          \n          \n          \n", buffer.get_contents());
    }
}
