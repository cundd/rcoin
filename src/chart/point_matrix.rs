use std::collections::BTreeMap;
use super::PointRow;
use super::Point;

pub trait PointMatrixTrait {
    fn get(&self, row: usize, column: usize) -> Option<Point>;
    fn has(&self, row: usize, column: usize) -> bool;
}

pub struct PointMatrix<'a> {
    rows: BTreeMap<usize, PointRow<'a>>,
}

impl<'a> PointMatrix<'a> {
    pub fn new(rows: BTreeMap<usize, PointRow<'a>>) -> PointMatrix {
        PointMatrix { rows }
    }

    fn get(&self, row: usize, column: usize) -> Option<Point> {
        if let Some(point_row) = self.rows.get(&row) {
            match point_row.get(&column) {
                Some(point) => Some(Point { x: point.x, y: point.y }),
                None => None,
            }
        } else {
            None
        }
    }

    fn has(&self, row: usize, column: usize) -> bool {
        if let Some(point_row) = self.rows.get(&row) {
            match point_row.get(&column) {
                Some(_) => true,
                None => false,
            }
        } else {
            false
        }
    }
}

impl<'a> PointMatrixTrait for PointMatrix<'a> {
    fn get(&self, row: usize, column: usize) -> Option<Point> {
        PointMatrix::get(self, row, column)
    }
    fn has(&self, row: usize, column: usize) -> bool {
        PointMatrix::has(self, row, column)
    }
}

impl<'a> PointMatrixTrait for &'a PointMatrix<'a> {
    fn get(&self, row: usize, column: usize) -> Option<Point> {
        PointMatrix::get(self, row, column)
    }

    fn has(&self, row: usize, column: usize) -> bool {
        PointMatrix::has(self, row, column)
    }
}