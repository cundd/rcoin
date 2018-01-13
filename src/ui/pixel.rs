use matrix::PointTrait;
use super::style::*;
use super::element::Element;

pub type CoordinatePrecision = u16;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pixel {
    x: CoordinatePrecision,
    y: CoordinatePrecision,
    element: Element,
    placeholder: bool,
}

#[allow(unused)]
impl Pixel {
    pub fn new(element: Element, x: CoordinatePrecision, y: CoordinatePrecision) -> Self {
        Pixel {
            element,
            x,
            y,
            placeholder: false,
        }
    }
    pub fn with_element_and_point<T: PointTrait>(element: Element, point: &T) -> Self {
        Pixel::new(
            element,
            point.x(),
             point.y(),
        )
    }

    pub fn normal_with_point<T: PointTrait>(character: char, point: &T) -> Self {
        Self::normal(character, point.x(), point.y())
    }

    pub fn normal(character: char, x: CoordinatePrecision, y: CoordinatePrecision) -> Self {
        Self::new(Element::normal(character), x, y)
    }

    pub fn placeholder(x: CoordinatePrecision, y: CoordinatePrecision) -> Self {
        Pixel { x, y, element: Element::blank(), placeholder: true }
    }


    pub fn blank(x: CoordinatePrecision, y: CoordinatePrecision) -> Self {
        Pixel { x, y, element: Element::blank(), placeholder: true }
    }

    pub fn blank_with_point<T: PointTrait>(point: &T) -> Self {
        Self::blank(point.x() as CoordinatePrecision, point.y() as CoordinatePrecision)
    }

    pub fn character(&self) -> char {
        self.element.character
    }

    pub fn styles(&self) -> Styles {
        self.element.styles
    }

    fn check_coordinate_precision(coordinate: usize) {
        if coordinate > CoordinatePrecision::max_value() as usize {
            panic!("ui::Pixel only supports u16 precision");
        }
    }
}

impl PointTrait for Pixel {
    fn x(&self) -> CoordinatePrecision {
        self.x as CoordinatePrecision
    }

    fn y(&self) -> CoordinatePrecision {
        self.y as CoordinatePrecision
    }

    fn with_x(&self, new_x: CoordinatePrecision) -> Self {
        let mut clone = self.clone();
        clone.x = new_x as CoordinatePrecision;

        clone
    }

    fn with_y(&self, new_y: CoordinatePrecision) -> Self {
        let mut clone = self.clone();
        clone.y = new_y as CoordinatePrecision;

        clone
    }

    fn with_x_y(&self, new_x: CoordinatePrecision, new_y: CoordinatePrecision) -> Self {
        let mut clone = self.clone();
        clone.x = new_x;
        clone.y = new_y;

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
                element: Element::normal('c'),
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
                element: Element::blank(),
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
                element: Element::blank(),
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