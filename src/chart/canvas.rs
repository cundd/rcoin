pub use super::point::Point;
use super::point_matrix::PointMatrixTrait;
use super::point_matrix::PointMatrix;

pub struct Canvas {
    width: usize,
    height: usize,
}


impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas { width, height }
    }

    pub fn draw_points(&self, matrix: PointMatrix) -> String {
        self.draw_points_with_callback(matrix, |point: Option<Point>| {
            match point {
                Some(_) => super::BLOCK_FULL.to_string(),
                None => " ".to_string(),
            }
        })
    }

    pub fn draw_points_with_symbol(&self, matrix: PointMatrix, symbol: &str) -> String {
        self.draw_points_with_callback(matrix, |point: Option<Point>| {
            match point {
                Some(_) => symbol.to_string(),
                None => " ".to_string(),
            }
        })
    }

    pub fn draw_points_with_symbols(&self, matrix: PointMatrix, point_symbol: &str, placeholder: &str) -> String {
        self.draw_points_with_callback(matrix, |point: Option<Point>| {
            match point {
                Some(_) => point_symbol.to_string(),
                None => placeholder.to_string(),
            }
        })
    }

    pub fn draw_points_with_callback<F>(&self, matrix: PointMatrix, draw_callback: F) -> String
        where F: Fn(Option<Point>) -> String {
        let mut buffer = String::with_capacity(self.width * self.height);

        for n in 0..self.height {
            buffer.push_str(&self.draw_row(n, &matrix, &draw_callback));
            buffer.push_str("\n");
        }

        buffer
    }

    fn draw_row<F>(&self, row: usize, matrix: &PointMatrix, draw_callback: &F) -> String
        where F: Fn(Option<Point>) -> String {
        let mut buffer = String::with_capacity(self.width);

        for column in 0..self.width {
            buffer.push_str(&draw_callback(matrix.get(row, column)));
        }

        buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draw_points_with_symbol_test() {
        let canvas = Canvas::new(10, 2);

        assert_eq!(
            ".         \n .        \n",
            canvas.draw_points_with_symbol(
                PointMatrix::new_from_vec(vec![
                    Point::new(0, 0),
                    Point::new(1, 1),
                    Point::new(2, 2),      // Will be clipped
                    Point::new(10, 20),    // Will be clipped
                    Point::new(12, 20),    // Will be clipped
                    Point::new(14, 20),    // Will be clipped
                    Point::new(11, 20),    // Will be clipped
                    Point::new(99, 20),    // Will be clipped
                    Point::new(100, 20),   // Will be clipped
                    Point::new(101, 20)    // Will be clipped
                ]),
                "."
            )
        );

        assert_eq!(
            "😊         \n 😊        \n",
            canvas.draw_points_with_symbol(
                PointMatrix::new_from_vec(vec![
                    Point::new(0, 0),
                    Point::new(1, 1),
                    Point::new(2, 2),      // Will be clipped
                    Point::new(10, 20),    // Will be clipped
                    Point::new(12, 20),    // Will be clipped
                    Point::new(14, 20),    // Will be clipped
                    Point::new(11, 20),    // Will be clipped
                    Point::new(99, 20),    // Will be clipped
                    Point::new(100, 20),   // Will be clipped
                    Point::new(101, 20)    // Will be clipped
                ]),
                "😊"
            )
        );

        assert_eq!(
            "😊         \n 😊       😊\n",
            canvas.draw_points_with_symbol(
                PointMatrix::new_from_vec(vec![
                    Point::new(0, 0),
                    Point::new(1, 1),
                    Point::new(9, 1),
                    Point::new(2, 2),      // Will be clipped
                    Point::new(10, 20),    // Will be clipped
                    Point::new(12, 20),    // Will be clipped
                    Point::new(14, 20),    // Will be clipped
                    Point::new(11, 20),    // Will be clipped
                    Point::new(99, 20),    // Will be clipped
                    Point::new(100, 20),   // Will be clipped
                    Point::new(101, 20)    // Will be clipped
                ]),
                "😊"
            )
        );
    }

    #[test]
    fn draw_points_with_symbols_test() {
        let canvas = Canvas::new(10, 2);

        assert_eq!(
            ".         \n .        \n",
            canvas.draw_points_with_symbols(
                PointMatrix::new_from_vec(vec![
                    Point::new(0, 0),
                    Point::new(1, 1),
                    Point::new(2, 2),      // Will be clipped
                    Point::new(10, 20),    // Will be clipped
                    Point::new(12, 20),    // Will be clipped
                    Point::new(14, 20),    // Will be clipped
                    Point::new(11, 20),    // Will be clipped
                    Point::new(99, 20),    // Will be clipped
                    Point::new(100, 20),   // Will be clipped
                    Point::new(101, 20)    // Will be clipped
                ]),
                ".",
                " "
            )
        );

        assert_eq!(
            "😊         \n 😊        \n",
            canvas.draw_points_with_symbols(
                PointMatrix::new_from_vec(vec![
                    Point::new(0, 0),
                    Point::new(1, 1),
                    Point::new(2, 2),      // Will be clipped
                    Point::new(10, 20),    // Will be clipped
                    Point::new(12, 20),    // Will be clipped
                    Point::new(14, 20),    // Will be clipped
                    Point::new(11, 20),    // Will be clipped
                    Point::new(99, 20),    // Will be clipped
                    Point::new(100, 20),   // Will be clipped
                    Point::new(101, 20)    // Will be clipped
                ]),
                "😊",
                " "
            )
        );

        assert_eq!(
            "😊_________\n_😊_______😊\n",
            canvas.draw_points_with_symbols(
                PointMatrix::new_from_vec(vec![
                    Point::new(0, 0),
                    Point::new(1, 1),
                    Point::new(9, 1),
                    Point::new(2, 2),      // Will be clipped
                    Point::new(10, 20),    // Will be clipped
                    Point::new(12, 20),    // Will be clipped
                    Point::new(14, 20),    // Will be clipped
                    Point::new(11, 20),    // Will be clipped
                    Point::new(99, 20),    // Will be clipped
                    Point::new(100, 20),   // Will be clipped
                    Point::new(101, 20)    // Will be clipped
                ]),
                "😊",
                "_"
            )
        );
    }

    #[test]
    fn draw_points_with_callback_test() {
        let canvas = Canvas::new(10, 2);

        assert_eq!(
            "x_________\n_x________\n",
            canvas.draw_points_with_callback(
                PointMatrix::new_from_vec(vec![
                    Point::new(0, 0),
                    Point::new(1, 1),
                    Point::new(2, 2),      // Will be clipped
                    Point::new(10, 20),    // Will be clipped
                    Point::new(12, 20),    // Will be clipped
                    Point::new(14, 20),    // Will be clipped
                    Point::new(11, 20),    // Will be clipped
                    Point::new(99, 20),    // Will be clipped
                    Point::new(100, 20),   // Will be clipped
                    Point::new(101, 20)    // Will be clipped
                ]),
                |point: Option<Point>| {
                    match point {
                        Some(_) => "x".to_string(),
                        None => "_".to_string(),
                    }
                }
            )
        );
    }
}
