use matrix::PointTrait;
use super::style::Style;

pub type CoordinatePrecision = u16;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pixel {
    pub x: CoordinatePrecision,
    pub y: CoordinatePrecision,
    pub character: char,
    pub style: Style,

    pub placeholder: bool,
}

#[allow(unused)]
impl Pixel {
    pub fn new(character: char, x: CoordinatePrecision, y: CoordinatePrecision, style: Style) -> Self {
        Pixel { x, y, character, style, placeholder: false }
    }

    pub fn with_point<T: PointTrait>(character: char, point: &T) -> Self {
        Self::check_coordinate_precision(point.x());
        Self::check_coordinate_precision(point.y());

        let x = point.x() as CoordinatePrecision;
        let y = point.x() as CoordinatePrecision;

        Self::new(character, x, y, Style::Normal)
    }

    pub fn normal(character: char, x: CoordinatePrecision, y: CoordinatePrecision) -> Self {
        Self::new(character, x, y, Style::Normal)
    }

    pub fn placeholder(x: CoordinatePrecision, y: CoordinatePrecision) -> Self {
        Pixel { x, y, character: ' ', style: Style::Normal, placeholder: true }
    }

    pub fn blank(x: CoordinatePrecision, y: CoordinatePrecision) -> Self {
        Pixel { x, y, character: ' ', style: Style::Normal, placeholder: true }
    }

    pub fn blank_with_point<T: PointTrait>(point: &T) -> Self {
        Self::check_coordinate_precision(point.x());
        Self::check_coordinate_precision(point.y());

        Self::blank(point.x() as CoordinatePrecision, point.y() as CoordinatePrecision)
    }

    fn check_coordinate_precision(coordinate: usize) {
        if coordinate > CoordinatePrecision::max_value() as usize {
            panic!("ui::Pixel only supports u16 precision");
        }
    }
}

impl PointTrait for Pixel {
    fn x(&self) -> usize {
        self.x as usize
    }

    fn y(&self) -> usize {
        self.y as usize
    }

    fn with_x(&self, new_x: usize) -> Self {
        Self::check_coordinate_precision(new_x);

        let mut clone = self.clone();
        clone.x = new_x as CoordinatePrecision;

        clone
    }

    fn with_y(&self, new_y: usize) -> Self {
        Self::check_coordinate_precision(new_y);

        let mut clone = self.clone();
        clone.y = new_y as CoordinatePrecision;

        clone
    }

    fn with_x_y(&self, new_x: usize, new_y: usize) -> Self {
        Self::check_coordinate_precision(new_x);
        Self::check_coordinate_precision(new_y);

        let mut clone = self.clone();
        clone.x = new_x as CoordinatePrecision;
        clone.y = new_y as CoordinatePrecision;

        clone
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use point::Point;

    #[test]
    fn default_test() {
        assert_eq!(
            Pixel {
                x: 1,
                y: 2,
                character: 'c',
                style: Style::Normal,
                placeholder: false,
            },
            Pixel::normal('c', 1, 2)
        );
    }

    #[test]
    fn blank_test() {
        assert_eq!(
            Pixel {
                x: 1,
                y: 2,
                character: ' ',
                style: Style::Normal,
                placeholder: true,
            },
            Pixel::blank(1, 2)
        );
    }


    #[test]
    fn blank_with_point() {
        assert_eq!(
            Pixel {
                x: 1,
                y: 2,
                character: ' ',
                style: Style::Normal,
                placeholder: true,
            },
            Pixel::blank_with_point(&Point::new(1, 2))
        );
    }

    #[test]
    fn with_x_test() {
        assert_eq!(30, Pixel::normal('c', 1, 2).with_x(30).x());
    }

    #[test]
    fn with_y_test() {
        assert_eq!(30, Pixel::normal('c', 1, 2).with_y(30).y());
    }

    #[test]
    fn with_x_y_test() {
        let pixel = Pixel::normal('c', 1, 2).with_x_y(30, 40);
        assert_eq!(30, pixel.x());
        assert_eq!(40, pixel.y());
    }
}