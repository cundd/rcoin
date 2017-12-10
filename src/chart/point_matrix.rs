use std::collections::BTreeMap;
use super::Point;

type PointRow<'a> = BTreeMap<usize, &'a Point>;

pub trait PointMatrixTrait {
    fn get(&self, row: usize, column: usize) -> Option<Point>;
    fn has(&self, row: usize, column: usize) -> bool;

    fn is_empty(&self) -> bool;
    fn x_max(&self) -> Option<usize>;
    fn y_max(&self) -> Option<usize>;
    fn x_y_max(&self) -> Option<(usize, usize)>;
}

#[derive(Debug)]
pub struct PointMatrix<'a> {
    rows: BTreeMap<usize, PointRow<'a>>,
}

impl<'a> PointMatrix<'a> {
    pub fn new(rows: BTreeMap<usize, PointRow<'a>>) -> Self {
        PointMatrix { rows }
    }

    pub fn new_from_vec(points: Vec<&'a Point>) -> Self {
        let mut rows: BTreeMap<usize, PointRow> = BTreeMap::new();

        for point in points {
            let mut handled = false;

            {
                let row_option = rows.get_mut(&point.y);
                if let Some(row) = row_option {
                    row.insert(point.x, point);
                    handled = true;
                }
            }
            if !handled {
                let mut row = PointRow::new();
                row.insert(point.x, point);
                rows.insert(point.y, row);
            }
        }

        PointMatrix::new(rows)
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

    fn x_max(&self) -> Option<usize> {
        if self.is_empty() {
            return None;
        }

        let mut x_max: usize = 0;
        for (_, row) in &self.rows {
            if let Some(&current_x) = row.keys().rev().nth(0) {
                if x_max < current_x {
                    x_max = current_x;
                }
            }
        }

        Some(x_max)
    }

    fn y_max(&self) -> Option<usize> {
        if let Some(y) = self.rows.keys().rev().nth(0) {
            Some(*y)
        } else {
            None
        }
    }

    fn x_y_max(&self) -> Option<(usize, usize)> {
        if self.is_empty() {
            return None;
        }

        Some((self.x_max().unwrap(), self.y_max().unwrap()))
    }

    fn is_empty(&self) -> bool {
        if self.rows.len() == 0 {
            return true;
        }

        self.rows.iter().find(|&(_, row)| row.len() > 0).is_none()
    }
}

impl<'a> PointMatrixTrait for PointMatrix<'a> {
    fn get(&self, row: usize, column: usize) -> Option<Point> {
        PointMatrix::get(self, row, column)
    }

    fn has(&self, row: usize, column: usize) -> bool {
        PointMatrix::has(self, row, column)
    }

    fn x_max(&self) -> Option<usize> {
        PointMatrix::x_max(self)
    }


    fn y_max(&self) -> Option<usize> {
        PointMatrix::y_max(self)
    }

    fn x_y_max(&self) -> Option<(usize, usize)> {
        PointMatrix::x_y_max(self)
    }

    fn is_empty(&self) -> bool {
        PointMatrix::is_empty(self)
    }
}

impl<'a> PointMatrixTrait for &'a PointMatrix<'a> {
    fn get(&self, row: usize, column: usize) -> Option<Point> {
        PointMatrix::get(self, row, column)
    }

    fn has(&self, row: usize, column: usize) -> bool {
        PointMatrix::has(self, row, column)
    }

    fn x_max(&self) -> Option<usize> {
        PointMatrix::x_max(self)
    }

    fn y_max(&self) -> Option<usize> {
        PointMatrix::y_max(self)
    }

    fn x_y_max(&self) -> Option<(usize, usize)> {
        PointMatrix::x_y_max(self)
    }

    fn is_empty(&self) -> bool {
        PointMatrix::is_empty(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_test_vec() -> Vec<Point> {
        vec![
            Point::new(0, 0),
            Point::new(1, 1),
            Point::new(2, 2),
            Point::new(10, 20),
            Point::new(12, 20),
            Point::new(14, 20),
            Point::new(11, 20),
            Point::new(99, 20),
            Point::new(100, 20),
            Point::new(101, 18)
        ]
    }

    fn build_min_test_vec() -> Vec<Point> {
        vec![
            Point::new(8, 10),
            Point::new(10, 4),
            Point::new(12, 5),
            Point::new(10, 20),
            Point::new(12, 20),
            Point::new(14, 20),
            Point::new(11, 20),
            Point::new(99, 20),
            Point::new(100, 20),
            Point::new(101, 18)
        ]
    }


    #[test]
    fn is_empty_test() {
        {
            let test_vec = build_test_vec();
            let matrix = PointMatrix::new_from_vec(test_vec.iter().collect());
            assert_eq!(false, matrix.is_empty());
        }
        {
            let test_vec = build_min_test_vec();
            let matrix = PointMatrix::new_from_vec(test_vec.iter().collect());
            assert_eq!(false, matrix.is_empty());
        }
        {
            assert!(PointMatrix::new(BTreeMap::new()).is_empty());
        }
        {
            let mut rows = BTreeMap::new();
            rows.insert(0, PointRow::new());
            rows.insert(1, PointRow::new());
            rows.insert(2, PointRow::new());
            rows.insert(3, PointRow::new());
            assert!(PointMatrix::new(rows).is_empty());
        }
    }

    #[test]
    fn x_max_test() {
        let test_vec = build_test_vec();
        let matrix = PointMatrix::new_from_vec(test_vec.iter().collect());
        assert_eq!(101, matrix.x_max().unwrap());
    }

    #[test]
    fn y_max_test() {
        let test_vec = build_test_vec();
        let matrix = PointMatrix::new_from_vec(test_vec.iter().collect());
        assert_eq!(20, matrix.y_max().unwrap());
    }

    #[test]
    fn x_y_max_test() {
        let test_vec = build_test_vec();
        let matrix = PointMatrix::new_from_vec(test_vec.iter().collect());
        assert_eq!((101, 20), matrix.x_y_max().unwrap());
    }
}
