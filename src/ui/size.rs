use super::pixel::CoordinatePrecision;

#[derive(Debug, Copy, Clone)]
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
}
