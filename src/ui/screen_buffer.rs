use point::Point;
use matrix::PointTrait;
use super::size::Size;
use super::pixel::Pixel;
use super::pixel::CoordinatePrecision;
use super::error::*;
use super::style;
use super::style::*;

#[derive(Debug)]
pub struct ScreenBuffer {
    size: Size,
    buffer: Vec<Pixel>,
    blank_pixel: Pixel,
}

#[allow(unused)]
impl ScreenBuffer {
    pub fn new(size: Size, blank_pixel: Pixel) -> Result<Self, Error> {
        Self::check_size(&size)?;

        let buffer = Vec::with_capacity(size.width as usize * size.height as usize);
        Ok(ScreenBuffer { size, buffer, blank_pixel })
    }

    pub fn with_size(size: Size) -> Result<Self, Error> {
        Self::new(size, Pixel::placeholder(0, 0))
    }

    pub fn width(&self) -> CoordinatePrecision {
        self.size.width
    }

    pub fn height(&self) -> CoordinatePrecision {
        self.size.height
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn draw_pixel(&mut self, pixel: Pixel) -> Result<(), Error> {
        let index = match self.get_index_for_point(&pixel) {
            Ok(i) => i,
            Err(e) => return Err(ui_error!(SizeError,"{} for character '{}'", e.to_string(), pixel.character))
        };
        if self.buffer.len() <= index {
            self.fill_buffer_up_to(index);
            self.buffer.push(pixel);
        } else {
            self.buffer[index as usize] = pixel;
        }

        Ok(())
    }

    pub fn draw_point<T: PointTrait>(&mut self, point: &T, content: char) -> Result<(), Error> {
        if point.x() as usize > CoordinatePrecision::max_value() as usize {
            return Err(ui_error!(SizeError,"The point's `x` coordinate ({}) exceeds the maximum Pixel `x` ({}) for character '{}'", point.x(), CoordinatePrecision::max_value(), content));
        }
        if point.y() as usize > CoordinatePrecision::max_value() as usize {
            return Err(ui_error!(SizeError,"The point's `y` coordinate ({}) exceeds the maximum Pixel `y` ({}) for character '{}'", point.y(), CoordinatePrecision::max_value(), content));
        }

        let x = point.x() as CoordinatePrecision;
        let y = point.y() as CoordinatePrecision;

        self.draw_pixel(Pixel::normal(content, x, y))
    }

    pub fn get_pixel_at_point<T: PointTrait>(&self, point: &T) -> Result<Pixel, Error> {
        let index = self.get_index_for_point(point)?;
        match self.get_pixel_at_index(index) {
            Some(c) => Ok(Pixel::new(c.character, c.x, c.y, c.styles)),
            None => Ok(self.blank_pixel.clone()),
        }
    }

    pub fn get_content_at_point<T: PointTrait>(&self, point: &T) -> Result<char, Error> {
        let pixel = self.get_pixel_at_point(point)?;

        Ok(pixel.character)
    }

    pub fn get_content_at(&self, x: CoordinatePrecision, y: CoordinatePrecision) -> Result<char, Error> {
        let pixel = self.get_pixel_at(x, y)?;

        Ok(pixel.character)
    }
    pub fn get_pixel_at(&self, x: CoordinatePrecision, y: CoordinatePrecision) -> Result<Pixel, Error> {
        let pixel = self.get_pixel_at_point(&Point::new(x, y))?;

        Ok(pixel)
    }

    pub fn get_contents(&self) -> String {
        build_contents(self, true)
    }

    #[inline]
    fn get_pixel_at_index(&self, index: usize) -> Option<&Pixel> {
        self.buffer.get(index)
    }

    /// Return the index of the last Point in the buffer
    fn last_index(&self) -> Option<usize> {
        match self.buffer.last() {
            Some(pixel) => match self.get_index_for_point(pixel) {
                Ok(index) => Some(index),
                Err(_) => None,
            },
            None => None,
        }
    }

    /// Returns the index of the last possible Point
    fn max_index(&self) -> usize {
        (self.size.width * self.size.height) as usize
    }

    fn check_size(size: &Size) -> Result<(), Error> {
        if size.width == 0 || size.height == 0 {
            return Err(ui_error!(SizeError, "The screen size {}x{} is too small", size.width, size.height));
        }

        let width: usize = size.width as usize;
        let height: usize = size.height as usize;

        // Number of available pixels
        if let Some(pixel_count) = width.checked_mul(height) {
            // Add a \n for each row
            if let Some(_) = pixel_count.checked_add(height) {
                return Ok(());
            }
        }
        Err(ui_error!(SizeError, "The screen size {}x{} is too big", size.width, size.height))
    }

    fn fill_buffer_up_to(&mut self, end: usize) {
        let mut index = self.buffer.len();
        while index < end {
            let y: CoordinatePrecision = f64::floor(index as f64 / self.size.width as f64) as CoordinatePrecision;
            let x_raw: usize = index - y as usize * self.size.width as usize;
            if x_raw >= CoordinatePrecision::max_value() as usize {
                panic!("Raw x value exceeds maximum Coordinate Precision");
            }

            let x = x_raw as CoordinatePrecision;

            self.buffer.push(Pixel::placeholder(x, y));

            index += 1;
        }
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

        Ok(
            point.y() as usize
                * self.size.width as usize
                + point.x() as usize
        )
    }
}

#[inline]
fn wrap_pixel(pixel: &Pixel) -> String {
    if pixel.styles == Styles::normal() {
        return pixel.character.to_string();
    }

    style::wrap(pixel.character, pixel.styles)
}

fn build_contents(buffer: &ScreenBuffer, with_colors: bool) -> String {
    let width = buffer.size.width;
    let height = buffer.size.height;
    let mut y = 0;
    let mut output = String::with_capacity(width as usize * height as usize + height as usize);

    while y < height {
        let mut current_index = y * width;
        let mut x = 0;
        while x < width {
            let pixel = buffer.get_pixel_at_point(&Point::new(x, y))
                .expect(&format!("No Pixel found for {}x{}", x, y));

            if with_colors {
                output.push_str(&wrap_pixel(&pixel));
            } else {
                output.push(pixel.character);
            }

            current_index += 1;
            x += 1;
        }
        output.push('\n');
        y += 1;
    }

    output
}


#[cfg(test)]
mod tests {
    use super::*;
    use point::Point;

    #[test]
    fn draw_point_should_fail_test() {
        assert_eq!(
            Err(Error::SizeError("The point's `x` coordinate (20000) is bigger than the screen's width (1) for character ' '".to_string())),
            ScreenBuffer::with_size(Size::new(1, 10)).unwrap()
                .draw_point(&Point::new(20_000, 0), ' ')
        );
        assert_eq!(
            Err(Error::SizeError("The point's `y` coordinate (30000) is bigger than the screen's height (1) for character ' '".to_string())),
            ScreenBuffer::with_size(Size::new(10, 1)).unwrap()
                .draw_point(&Point::new(0, 30_000), ' ')
        );
        assert_eq!(
            Err(Error::SizeError("The point's `x` coordinate (10) is bigger than the screen's width (10) for character ' '".to_string())),
            ScreenBuffer::with_size(Size::new(10, 10)).unwrap()
                .draw_point(&Point::new(10, 10), ' ')
        );

//        // CoordinatePrecision overflow
//        assert_eq!(
//            Err(Error::SizeError("The point's `x` coordinate (2000000) exceeds the maximum Pixel `x` (65535) for character ' '".to_string())),
//            ScreenBuffer::with_size(Size::new(1, 10)).unwrap()
//                .draw_point(&Point::new(2_000_000, 0), ' ')
//        );
//        assert_eq!(
//            Err(Error::SizeError("The point's `y` coordinate (3000000) exceeds the maximum Pixel `y` (65535) for character ' '".to_string())),
//            ScreenBuffer::with_size(Size::new(10, 1)).unwrap()
//                .draw_point(&Point::new(0, 3_000_000), ' ')
//        );
    }

    #[test]
    fn draw_point_test() {
        let mut buffer = ScreenBuffer::with_size(Size::new(3_200, 1_024)).unwrap();
        let point = Point::new(100, 200);

//        let buffer_size_before_draw = buffer.buffer.len();
        assert!(buffer.draw_point(&point, 'x').is_ok());
        assert_eq!('x', buffer.get_content_at_point(&point).expect(&format!("Failed to fetch content at {}", point)));

//        assert_eq!(buffer_size_before_draw, buffer.buffer.len());
        assert_eq!(100 + 200 * 3_200 + 1, buffer.buffer.len());
    }

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
        assert_eq!("  \n", build_contents(&ScreenBuffer::with_size(Size::new(2, 1)).unwrap(), false));
//        assert_eq!("  \n", ScreenBuffer::with_size(Size::new(2, 1)).unwrap().get_contents());
        assert_eq!("  \n  \n", build_contents(&ScreenBuffer::with_size(Size::new(2, 2)).unwrap(), false));
//        assert_eq!("  \n  \n", ScreenBuffer::with_size(Size::new(2, 2)).unwrap().get_contents());
    }

    #[test]
    fn get_contents_test() {
        let mut buffer = ScreenBuffer::with_size(Size::new(2, 2)).unwrap();
        assert!(buffer.draw_point(&Point::new(0, 0), 'x').is_ok());
        assert_eq!("x \n  \n", build_contents(&buffer, false));
//        assert_eq!("x \n  \n", buffer.get_contents());

        let mut buffer = ScreenBuffer::with_size(Size::new(10, 20)).unwrap();
        assert!(buffer.draw_point(&Point::new(5, 10), 'x').is_ok());
        assert_eq!("          \n          \n          \n          \n          \n          \n          \n          \n          \n          \n     x    \n          \n          \n          \n          \n          \n          \n          \n          \n          \n", build_contents(&buffer, false));
//        assert_eq!("          \n          \n          \n          \n          \n          \n          \n          \n          \n          \n     x    \n          \n          \n          \n          \n          \n          \n          \n          \n          \n", buffer.get_contents());
    }
}
