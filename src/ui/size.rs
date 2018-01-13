use term_size;
use super::pixel::CoordinatePrecision;
use super::error::Error;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Size {
    pub width: CoordinatePrecision,
    pub height: CoordinatePrecision,
}

impl Size {
    pub fn new(width: CoordinatePrecision, height: CoordinatePrecision) -> Size {
        if width == 0 {
            panic!("Width must be bigger than 0");
        }
        if height == 0 {
            panic!("Height must be bigger than 0");
        }
        Size { width, height }
    }

    pub fn auto() -> Result<Size, Error> {
        if let Some((width, height)) = term_size::dimensions() {
            Ok(Size::new(width as CoordinatePrecision, height as CoordinatePrecision))
        } else {
            Err(ui_error!(SizeError, "Could not determine the screen size"))
        }
    }
}
