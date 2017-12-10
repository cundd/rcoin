#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }

    pub fn null() -> Self {
        Point {
            x: 0,
            y: 0,
        }
    }
}
