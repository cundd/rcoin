/// The Point is currently only meant for testing purpose
#[cfg(test)]
mod internal {
    use matrix::PointTrait;

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct Point {
        pub x: usize,
        pub y: usize,
    }

    impl Point {
        #[allow(unused)]
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

    impl PointTrait for Point {
        fn x(&self) -> usize {
            self.x
        }

        fn y(&self) -> usize {
            self.y
        }

        fn with_x(&self, new_x: usize) -> Self {
            let mut clone = self.clone();
            clone.x = new_x;

            clone
        }

        fn with_y(&self, new_y: usize) -> Self {
            let mut clone = self.clone();
            clone.y = new_y;

            clone
        }

        fn with_x_y(&self, new_x: usize, new_y: usize) -> Self {
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
}

#[cfg(test)]
pub use self::internal::Point;
