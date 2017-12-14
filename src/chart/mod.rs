mod canvas;
mod mode;
mod point;
mod point_matrix;
mod transform;

pub use self::point::Point;
pub use self::mode::Mode;
use self::canvas::Canvas;
use self::point_matrix::PointMatrix;

#[allow(dead_code)]
const BLOCK_FULL: &'static str = "\u{2588}";
#[allow(dead_code)]
const BLOCK_UPPER_HALF: &'static str = "\u{2580}";
#[allow(dead_code)]
const BLOCK_LOWER_HALF: &'static str = "\u{2584}";

pub struct Chart {
    mode: Mode,
    width: usize,
    height: usize,
}

impl Chart {
    pub fn new(width: usize, height: usize, mode: Mode) -> Self {
        let canvas = Canvas::new(width, height, 0, 0);
        Chart { width, height, mode }
    }

    #[allow(dead_code)]
    pub fn draw_points(&self, points: Vec<Point>) -> String {
        let matrix = PointMatrix::new_from_vec(points);
        if let Some(canvas) = self.get_canvas(&matrix) {
            match self.mode {
                Mode::Truncate => canvas.draw_points(matrix),
                Mode::ScaleX => canvas.draw_points(transform::scale_x(self, &matrix)),
                Mode::ScaleY => canvas.draw_points(transform::scale_y(self, &matrix)),
                Mode::Scale => canvas.draw_points(transform::scale(self, &matrix)),
                Mode::ScaleDownX => canvas.draw_points(transform::scale_down_x(self, &matrix)),
                Mode::ScaleDownY => canvas.draw_points(transform::scale_down_y(self, &matrix)),
                Mode::ScaleDown => canvas.draw_points(transform::scale_down(self, &matrix)),
            }
        } else {
            "".to_string()
        }
    }

    #[allow(dead_code)]
    pub fn draw_points_with_symbol(&self, points: Vec<Point>, symbol: &str) -> String {
        let matrix = PointMatrix::new_from_vec(points);
        if let Some(canvas) = self.get_canvas(&matrix) {
            match self.mode {
                Mode::Truncate => canvas.draw_points_with_symbol(matrix, symbol),
                Mode::ScaleX => canvas.draw_points_with_symbol(transform::scale_x(self, &matrix), symbol),
                Mode::ScaleY => canvas.draw_points_with_symbol(transform::scale_y(self, &matrix), symbol),
                Mode::Scale => canvas.draw_points_with_symbol(transform::scale(self, &matrix), symbol),
                Mode::ScaleDownX => canvas.draw_points_with_symbol(transform::scale_down_x(self, &matrix), symbol),
                Mode::ScaleDownY => canvas.draw_points_with_symbol(transform::scale_down_y(self, &matrix), symbol),
                Mode::ScaleDown => canvas.draw_points_with_symbol(transform::scale_down(self, &matrix), symbol),
            }
        } else {
            "".to_string()
        }
    }

    #[allow(dead_code)]
    pub fn draw_points_with_symbols(&self, points: Vec<Point>, point_symbol: &str, placeholder: &str) -> String {
        let matrix = PointMatrix::new_from_vec(points);
        if let Some(canvas) = self.get_canvas(&matrix) {
            match self.mode {
                Mode::Truncate => canvas.draw_points_with_symbols(matrix, point_symbol, placeholder),
                Mode::ScaleX => canvas.draw_points_with_symbols(transform::scale_x(self, &matrix), point_symbol, placeholder),
                Mode::ScaleY => canvas.draw_points_with_symbols(transform::scale_y(self, &matrix), point_symbol, placeholder),
                Mode::Scale => canvas.draw_points_with_symbols(transform::scale(self, &matrix), point_symbol, placeholder),
                Mode::ScaleDownX => canvas.draw_points_with_symbols(transform::scale_down_x(self, &matrix), point_symbol, placeholder),
                Mode::ScaleDownY => canvas.draw_points_with_symbols(transform::scale_down_y(self, &matrix), point_symbol, placeholder),
                Mode::ScaleDown => canvas.draw_points_with_symbols(transform::scale_down(self, &matrix), point_symbol, placeholder),
            }
        } else {
            "".to_string()
        }
    }

    #[allow(dead_code)]
    pub fn draw_points_with_callback<F>(&self, points: Vec<Point>, draw_callback: F) -> String
        where F: Fn(Option<Point>) -> String {
        let matrix = PointMatrix::new_from_vec(points);
        if let Some(canvas) = self.get_canvas(&matrix) {
            match self.mode {
                Mode::Truncate => canvas.draw_points_with_callback(matrix, &draw_callback),
                Mode::ScaleX => canvas.draw_points_with_callback(transform::scale_x(self, &matrix), &draw_callback),
                Mode::ScaleY => canvas.draw_points_with_callback(transform::scale_y(self, &matrix), &draw_callback),
                Mode::Scale => canvas.draw_points_with_callback(transform::scale(self, &matrix), &draw_callback),
                Mode::ScaleDownX => canvas.draw_points_with_callback(transform::scale_down_x(self, &matrix), &draw_callback),
                Mode::ScaleDownY => canvas.draw_points_with_callback(transform::scale_down_y(self, &matrix), &draw_callback),
                Mode::ScaleDown => canvas.draw_points_with_callback(transform::scale_down(self, &matrix), &draw_callback),
            }
        } else {
            "".to_string()
        }
    }

    fn get_canvas(&self, point_matrix: &PointMatrix) -> Option<Canvas> {
        let x_start = point_matrix.x_min();
        let y_start = point_matrix.y_min();
        if x_start.is_none() || y_start.is_none() {
            return None;
        }
        Some(Canvas::new(self.width, self.height, x_start.unwrap(), y_start.unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draw_points_with_symbol_test() {
        let canvas = Chart::new(10, 2, Mode::Truncate);

        assert_eq!(
            " .        \n.         \n",
            canvas.draw_points_with_symbol(
                vec![
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
                ],
                "."
            )
        );

        assert_eq!(
            " 😊        \n😊         \n",
            canvas.draw_points_with_symbol(
                vec![
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
                ],
                "😊"
            )
        );

        assert_eq!(
            " 😊       😊\n😊         \n",
            canvas.draw_points_with_symbol(
                vec![
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
                ],
                "😊"
            )
        );
    }

    #[test]
    fn draw_points_with_symbols_test() {
        let canvas = Chart::new(10, 2, Mode::Truncate);

        assert_eq!(
            " .        \n.         \n",
            canvas.draw_points_with_symbols(
                vec![
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
                ],
                ".",
                " "
            )
        );

        assert_eq!(
            " 😊        \n😊         \n",
            canvas.draw_points_with_symbols(
                vec![
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
                ],
                "😊",
                " "
            )
        );

        assert_eq!(
            "_😊_______😊\n😊_________\n",
            canvas.draw_points_with_symbols(
                vec![
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
                ],
                "😊",
                "_"
            )
        );
    }

    #[test]
    fn draw_points_with_callback_test() {
        let canvas = Chart::new(10, 2, Mode::Truncate);

        assert_eq!(
            "_x________\nx_________\n",
            canvas.draw_points_with_callback(
                vec![
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
                ],
                |point: Option<Point>| {
                    match point {
                        Some(_) => "x".to_string(),
                        None => "_".to_string(),
                    }
                }
            )
        );

//        let canvas = Chart::new(10, 2, Mode::Truncate);
//        assert_eq!(
//            "x_________\n_x________\n",
//            canvas.draw_points_with_callback(
//                vec![
//                    Point::new(0, 1000),
//                ],
//                |point: Option<Point>| {
//                    match point {
//                        Some(_) => "x".to_string(),
//                        None => "_".to_string(),
//                    }
//                }
//            )
//        );
    }
}
