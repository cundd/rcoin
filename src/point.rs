use matrix::PointTrait;
use ui::CoordinatePrecision;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: CoordinatePrecision,
    pub y: CoordinatePrecision,
}

impl Point {
    #[allow(unused)]
    pub fn new(x: CoordinatePrecision, y: CoordinatePrecision) -> Self {
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

impl PointTrait for Point {
    fn x(&self) -> CoordinatePrecision {
        self.x
    }

    fn y(&self) -> CoordinatePrecision {
        self.y
    }

    fn with_x(&self, new_x: CoordinatePrecision) -> Self {
        let mut clone = self.clone();
        clone.x = new_x;

        clone
    }

    fn with_y(&self, new_y: CoordinatePrecision) -> Self {
        let mut clone = self.clone();
        clone.y = new_y;

        clone
    }

    fn with_x_y(&self, new_x: CoordinatePrecision, new_y: CoordinatePrecision) -> Self {
        let mut clone = self.clone();
        clone.x = new_x;
        clone.y = new_y;

        clone
    }
}

impl ::std::fmt::Display for Point {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _ = write!(f, "Point {}x{}", self.x, self.y);
        Ok(())
    }
}
