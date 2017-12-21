pub mod color;
pub mod mode;
pub mod point;
pub mod point_drawing;
pub mod configuration;
mod canvas;
mod transform;

use term_size;
pub use self::point::Point;
pub use self::mode::Mode;
use self::canvas::Canvas;
use matrix::Matrix;

#[allow(unused)]
pub const BLOCK_FULL: &'static str = "\u{2588}";
#[allow(unused)]
pub const BLOCK_UPPER_HALF: &'static str = "\u{2580}";
#[allow(unused)]
pub const BLOCK_LOWER_HALF: &'static str = "\u{2584}";

pub struct Chart {
    mode: Mode,
    _width: usize,
    _height: usize,
    pub y_scala_width: usize,
    pub x_scala_width: usize,
}

impl Chart {
    pub fn new(width: usize, height: usize, x_scala_width: usize, y_scala_width: usize, mode: Mode) -> Self {
        Chart { _width: width, _height: height, mode, x_scala_width, y_scala_width }
    }

    pub fn width(&self) -> usize {
        if self._width > 0 {
            self._width
        } else {
            match term_size::dimensions() {
                Some((default, _)) => default,
                None => 30,
            }
        }
    }

    #[allow(unused)]
    pub fn height(&self) -> usize {
        if self._height > 0 {
            self._height
        } else {
            match term_size::dimensions() {
                Some((_, default)) => default,
                None => 10,
            }
        }
    }

    #[allow(unused)]
    pub fn draw_points(&self, points: Vec<Point>) -> String {
        let matrix = Matrix::from_vec(points);
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

    #[allow(unused)]
    pub fn draw_points_with_configuration(&self, points: Vec<Point>, conf: &self::configuration::Configuration) -> String {
        let matrix = Matrix::from_vec(points);
        if let Some(canvas) = self.get_canvas(&matrix) {
            match self.mode {
                Mode::Truncate => canvas.draw_points_with_configuration(matrix, conf),
                Mode::ScaleX => canvas.draw_points_with_configuration(transform::scale_x(self, &matrix), conf),
                Mode::ScaleY => canvas.draw_points_with_configuration(transform::scale_y(self, &matrix), conf),
                Mode::Scale => canvas.draw_points_with_configuration(transform::scale(self, &matrix), conf),
                Mode::ScaleDownX => canvas.draw_points_with_configuration(transform::scale_down_x(self, &matrix), conf),
                Mode::ScaleDownY => canvas.draw_points_with_configuration(transform::scale_down_y(self, &matrix), conf),
                Mode::ScaleDown => canvas.draw_points_with_configuration(transform::scale_down(self, &matrix), conf),
            }
        } else {
            "".to_string()
        }
    }

    #[allow(unused)]
    pub fn draw_points_with_symbol(&self, points: Vec<Point>, symbol: &str) -> String {
        let matrix = Matrix::from_vec(points);
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

    #[allow(unused)]
    pub fn draw_points_with_symbols(&self, points: Vec<Point>, point_symbol: &str, placeholder: &str) -> String {
        let matrix = Matrix::from_vec(points);
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

    #[allow(unused)]
    pub fn draw_points_with_callback<F>(&self, points: Vec<Point>, draw_callback: F) -> String
        where F: Fn(Option<Point>) -> String {
        let matrix = Matrix::from_vec(points);
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

    fn get_canvas(&self, point_matrix: &Matrix<Point>) -> Option<Canvas> {
        let x_min_option = point_matrix.x_min();
        let y_min_option = point_matrix.y_min();
        if x_min_option.is_none() || y_min_option.is_none() {
            return None;
        }
        let x_min = x_min_option.unwrap();
        let y_min = y_min_option.unwrap();
        let x_start = if x_min > 0 { x_min - 1 } else { 0 };
        let y_start = if y_min > 0 { y_min - 1 } else { 0 };

        Some(Canvas::new(self.width() - self.y_scala_width, self.height() - self.x_scala_width, x_start, y_start))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draw_points_with_symbol_test() {
        let canvas = Chart::new(10, 2, 0, 0, Mode::Truncate);

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
                ".",
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
                "😊",
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
                "😊",
            )
        );
    }

    #[test]
    fn draw_points_with_symbols_test() {
        let canvas = Chart::new(10, 2, 0, 0, Mode::Truncate);

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
                " ",
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
                " ",
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
                "_",
            )
        );
    }

    #[test]
    fn draw_points_with_callback_test() {
        let canvas = Chart::new(10, 2, 0, 0, Mode::Truncate);

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
                },
            )
        );
    }

    #[test]
    fn scale_not_needed_test() {
        let chart_with_truncate = Chart::new(6, 6, 0, 0, Mode::Truncate).draw_points_with_symbols(
            vec![
                Point::new(0, 0),
                Point::new(5, 5),
            ],
            "x", "_",
        );
        let chart_with_scale = Chart::new(6, 6, 0, 0, Mode::Scale).draw_points_with_symbols(
            vec![
                Point::new(0, 0),
                Point::new(5, 5),
            ],
            "x", "_",
        );

        assert_eq!(
            chart_with_truncate,
            chart_with_scale
        );
    }

    #[test]
    fn scale_test() {
        let chart_with_truncate = Chart::new(6, 6, 0, 0, Mode::Truncate).draw_points_with_symbols(
            vec![
                Point::new(0, 0),
                Point::new(2, 2),
                Point::new(5, 5),
            ],
            "x", "_",
        );
        let chart_with_scale = Chart::new(6, 6, 0, 0, Mode::Scale).draw_points_with_symbols(
            vec![
                Point::new(0, 0),
                Point::new(5, 5),
                Point::new(11, 11),
            ],
            "x", "_",
        );

        assert_eq!(
            chart_with_truncate,
            chart_with_scale
        );

        assert_eq!(
            chart_with_truncate,
            Chart::new(6, 6, 0, 0, Mode::ScaleDown).draw_points_with_symbols(
                vec![
                    Point::new(0, 0),
                    Point::new(5, 5),
                    Point::new(11, 11),
                ],
                "x", "_",
            )
        );
    }

    #[test]
    fn scale_down_test() {
        let chart_with_truncate = Chart::new(6, 6, 0, 0, Mode::Truncate).draw_points_with_symbols(
            vec![
                Point::new(0, 0),
                Point::new(2, 2),
                Point::new(5, 5),
            ],
            "x", "_",
        );

        assert_eq!(
            chart_with_truncate,
            Chart::new(6, 6, 0, 0, Mode::Scale).draw_points_with_symbols(
                vec![
                    Point::new(0, 0),
                    Point::new(5, 5),
                    Point::new(11, 11),
                ],
                "x", "_",
            )
        );

        assert_eq!(
            chart_with_truncate,
            Chart::new(6, 6, 0, 0, Mode::ScaleDown).draw_points_with_symbols(
                vec![
                    Point::new(0, 0),
                    Point::new(5, 5),
                    Point::new(11, 11),
                ],
                "x", "_",
            )
        );
    }

    #[test]
    fn do_not_scale_up_test() {
        assert_eq!(
            Chart::new(6, 6, 0, 0, Mode::Truncate).draw_points_with_symbols(
                vec![
                    Point::new(0, 0),
                    Point::new(2, 2),
                ],
                "x", "_",
            ),
            Chart::new(6, 6, 0, 0, Mode::ScaleDown).draw_points_with_symbols(
                vec![
                    Point::new(0, 0),
                    Point::new(2, 2),
                ],
                "x", "_",
            )
        );
        assert_ne!(
            Chart::new(6, 6, 0, 0, Mode::Truncate).draw_points_with_symbols(
                vec![
                    Point::new(0, 0),
                    Point::new(2, 2),
                ],
                "x", "_",
            ),
            Chart::new(6, 6, 0, 0, Mode::Scale).draw_points_with_symbols(
                vec![
                    Point::new(0, 0),
                    Point::new(2, 2),
                ],
                "x", "_",
            )
        );
    }
}
