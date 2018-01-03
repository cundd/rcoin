#[derive(Debug, Copy, Clone)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

impl Size {
    pub fn new(width: usize, height: usize) -> Size {
        if width == 0 {
            panic!("Width must be bigger than 0");
        }
        if height == 0 {
            panic!("Height must be bigger than 0");
        }
        Size { width, height }
    }
}
