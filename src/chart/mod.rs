use std::collections::BTreeMap;

mod point;
mod point_matrix;

pub use self::point::Point;

type PointRow<'a> = BTreeMap<usize, &'a Point>;

use self::point_matrix::PointMatrixTrait;
use self::point_matrix::PointMatrix;


pub struct Chart {
    width: usize,
    height: usize,
}

const BLOCK_FULL: &'static str = "\u{2588}";
const BLOCK_UPPER_HALF: &'static str = "\u{2580}";
const BLOCK_LOWER_HALF: &'static str = "\u{2584}";

fn sort_points<'a>(chart: &Chart, points: Vec<&'a Point>) -> PointMatrix<'a> {
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


impl Chart {
    pub fn new(width: usize, height: usize) -> Self {
        Chart { width, height }
    }

    pub fn draw_points(&self, points: Vec<&Point>) {
        let matrix = sort_points(self, points);

        for n in 0..self.height {
            self.draw_row(n, &matrix);
            println!();
        }
    }

    pub fn draw_point(&self, point: &Point) {
        self.draw_points(vec![point]);
    }

    fn draw_row(&self, row: usize, matrix: &PointMatrix) {
        for column in 0..self.width {
            if let Some(p) = matrix.get(row, column) {
                print!("{}", BLOCK_FULL);
            } else {
                print!(" ");
            }
        }
    }
}
