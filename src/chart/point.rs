use matrix::CoordinatesTrait;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }

    #[allow(unused)]
    pub fn null() -> Self {
        Point {
            x: 0,
            y: 0,
        }
    }
}

impl CoordinatesTrait for Point {
    fn x(&self) -> usize {
        self.x
    }

    fn y(&self) -> usize {
        self.y
    }
}
