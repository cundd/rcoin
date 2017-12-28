use std::collections::BTreeMap;
use std::fmt::Debug;

pub type Row<I: Debug> = BTreeMap<usize, I>;

pub trait PointTrait: Clone {
    fn x(&self) -> usize;
    fn y(&self) -> usize;

    fn with_x(&self, new_x: usize) -> Self;
    fn with_y(&self, new_y: usize) -> Self;
    fn with_x_y(&self, new_x: usize, new_y: usize) -> Self;
}

#[derive(Debug, Clone)]
pub struct Matrix<I: Clone + PointTrait> {
    len: usize,
    pub  rows: BTreeMap<usize, Row<I>>,
}

impl<I: Clone + PointTrait> Matrix<I> {
    pub fn from_slice(points: &[I]) -> Self {
        let mut rows: BTreeMap<usize, Row<I>> = BTreeMap::new();
        let len = points.len();

        for point in points {
            if rows.contains_key(&point.y()) {
                let row_option = rows.get_mut(&point.y());
                if let Some(row) = row_option {
                    row.insert(point.x(), point.clone());
                }
            } else {
                let mut row = Row::new();
                let y = point.y();
                row.insert(point.x(), point.clone());
                rows.insert(y, row);
            }
        }

        Matrix { len, rows }
    }

    pub fn from_vec(points: Vec<I>) -> Self {
        Self::from_slice(&points)
    }


    #[allow(unused)]
    fn new(len: usize, rows: BTreeMap<usize, Row<I>>) -> Self {
        Matrix { len, rows }
    }

    pub fn map<F>(&self, callback: F) -> Self
        where F: Fn(I) -> I {
        let mut temp_vec: Vec<I> = Vec::with_capacity(self.len);
        for (_, row) in &self.rows {
            for (_, point) in row {
                temp_vec.push(callback(point.clone()));
            }
        }

        Matrix::from_vec(temp_vec)
    }

    pub fn get(&self, row: usize, column: usize) -> Option<I> {
        if let Some(row) = self.rows.get(&row) {
            match row.get(&column) {
                Some(point) => Some(point.clone()),
                None => None,
            }
        } else {
            None
        }
    }

    pub fn get_row(&self, row: usize) -> Option<&Row<I>> {
        self.rows.get(&row)
    }

    #[allow(unused)]
    pub fn has(&self, row: usize, column: usize) -> bool {
        if let Some(point_row) = self.rows.get(&row) {
            match point_row.get(&column) {
                Some(_) => true,
                None => false,
            }
        } else {
            false
        }
    }

    pub fn x_max(&self) -> Option<usize> {
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

    pub fn y_max(&self) -> Option<usize> {
        if let Some(y) = self.rows.keys().rev().nth(0) {
            Some(*y)
        } else {
            None
        }
    }

    #[allow(unused)]
    pub fn x_y_max(&self) -> Option<(usize, usize)> {
        if self.is_empty() {
            return None;
        }

        Some((self.x_max().unwrap(), self.y_max().unwrap()))
    }

    pub fn is_empty(&self) -> bool {
        return self.len == 0;
    }

    pub fn x_min(&self) -> Option<usize> {
        if self.is_empty() {
            return None;
        }

        let mut x_min: usize = usize::max_value();
        for (_, row) in &self.rows {
            if let Some(&current_x) = row.keys().nth(0) {
                if x_min > current_x {
                    x_min = current_x;
                }
            }
        }

        Some(x_min)
    }

    pub fn y_min(&self) -> Option<usize> {
        if self.is_empty() {
            return None;
        }

        if let Some(y) = self.rows.keys().nth(0) {
            Some(*y)
        } else {
            None
        }
    }

    #[allow(unused)]
    pub fn x_y_min(&self) -> Option<(usize, usize)> {
        if self.is_empty() {
            return None;
        }

        Some((self.x_min().unwrap(), self.y_min().unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    struct Point {
        x: usize,
        y: usize,
    }

    impl Point {
        fn new(x: usize, y: usize) -> Self {
            Point { x, y }
        }
    }

    impl PointTrait for Point {
        fn x(&self) -> usize { self.x }
        fn y(&self) -> usize { self.y }
        fn with_x(&self, _: usize) -> Self { unimplemented!(); }
        fn with_y(&self, _: usize) -> Self { unimplemented!(); }
        fn with_x_y(&self, _: usize, _: usize) -> Self { unimplemented!(); }
    }

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
            let matrix = Matrix::from_vec(build_test_vec());
            assert_eq!(false, matrix.is_empty());
        }
        {
            let matrix = Matrix::from_vec(build_min_test_vec());
            assert_eq!(false, matrix.is_empty());
        }
        {
            assert!(Matrix::new(0, BTreeMap::<usize, Row<Point>>::new()).is_empty());
        }
        {
            let mut rows = BTreeMap::new();
            rows.insert(0, Row::<Point>::new());
            rows.insert(1, Row::<Point>::new());
            rows.insert(2, Row::<Point>::new());
            rows.insert(3, Row::<Point>::new());
            assert!(Matrix::new(0, rows).is_empty());
        }
    }

    #[test]
    fn x_max_test() {
        let matrix = Matrix::from_vec(build_test_vec());
        assert_eq!(101, matrix.x_max().unwrap());
    }

    #[test]
    fn y_max_test() {
        let matrix = Matrix::from_vec(build_test_vec());
        assert_eq!(20, matrix.y_max().unwrap());
    }

    #[test]
    fn x_y_max_test() {
        let matrix = Matrix::from_vec(build_test_vec());
        assert_eq!((101, 20), matrix.x_y_max().unwrap());
    }

    #[test]
    fn x_min_test() {
        let matrix = Matrix::from_vec(build_min_test_vec());
        assert_eq!(8, matrix.x_min().unwrap());
    }

    #[test]
    fn y_min_test() {
        let matrix = Matrix::from_vec(build_min_test_vec());
        assert_eq!(4, matrix.y_min().unwrap());
    }

    #[test]
    fn x_y_min_test() {
        let matrix = Matrix::from_vec(build_min_test_vec());
        assert_eq!((8, 4), matrix.x_y_min().unwrap());
    }

    #[test]
    fn map_test() {
        let matrix = Matrix::from_vec(vec![
            Point::new(0, 10),
            Point::new(10, 20),
        ]).map(|p| { Point::new(p.x + 5, p.y + 7) });
        assert!(matrix.get(17, 5).is_some());
        assert!(matrix.get(27, 15).is_some());
    }
}
