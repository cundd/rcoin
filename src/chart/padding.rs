#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Padding {
    pub top: usize,
    pub right: usize,
    pub bottom: usize,
    pub left: usize,
}

impl Padding {
    pub fn new(top: usize, right: usize, bottom: usize, left: usize) -> Self {
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
