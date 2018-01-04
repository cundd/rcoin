use ui::CoordinatePrecision;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Padding {
    pub top: CoordinatePrecision,
    pub right: CoordinatePrecision,
    pub bottom: CoordinatePrecision,
    pub left: CoordinatePrecision,
}

impl Padding {
    pub fn new(top: CoordinatePrecision, right: CoordinatePrecision, bottom: CoordinatePrecision, left: CoordinatePrecision) -> Self {
        Padding {
            top,
            right,
            bottom,
            left,
        }
    }

    #[allow(unused)]
    pub fn empty() -> Self {
        Padding {
            top: 0,
            right: 0,
            bottom: 0,
            left: 0,
        }
    }
}
