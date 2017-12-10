use std::collections::BTreeMap;

mod point;
mod point_matrix;

pub use self::point::Point;

type PointRow<'a> = BTreeMap<usize, &'a Point>;

use self::point_matrix::PointMatrixTrait;
use self::point_matrix::PointMatrix;


//pub type DrawCallback = Fn(point: & Point) -> usize ;


pub struct Chart {
    width: usize,
    height: usize,
}

const BLOCK_FULL: &'static str = "\u{2588}";
const BLOCK_UPPER_HALF: &'static str = "\u{2580}";
const BLOCK_LOWER_HALF: &'static str = "\u{2584}";

impl Chart {
    pub fn new(width: usize, height: usize) -> Self {
        Chart { width, height }
    }

    #[allow(dead_code)]
    pub fn draw_points(&self, points: Vec<&Point>) {
        self.draw_points_with_callback(points, |point: Option<Point>| {
            match point {
                Some(_) => print!("{}", BLOCK_FULL),
                None => print!(" "),
            }
        })
    }

    #[allow(dead_code)]
    pub fn draw_points_with_symbol(&self, points: Vec<&Point>, symbol: &str) {
        self.draw_points_with_callback(points, |point: Option<Point>| {
            match point {
                Some(_) => print!("{}", symbol),
                None => print!(" "),
            }
        })
    }

    #[allow(dead_code)]
    pub fn draw_points_with_callback<F>(&self, points: Vec<&Point>, draw_callback: F)
        where F: Fn(Option<Point>) {
        let matrix = PointMatrix::new_from_vec(points);

        for n in 0..self.height {
            self.draw_row(n, &matrix, &draw_callback);
            println!();
        }
    }

    fn draw_row<F>(&self, row: usize, matrix: &PointMatrix, draw_callback: &F)
        where F: Fn(Option<Point>) {
        for column in 0..self.width {
            draw_callback(matrix.get(row, column));
//            if let Some(p) = matrix.get(row, column) {
//            } else {
//                print!(" ");
//            }
        }
    }
}
