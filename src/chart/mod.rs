mod canvas;
mod point;
mod point_matrix;
mod transform;

pub use self::point::Point;
use self::canvas::Canvas;
use self::point_matrix::PointMatrix;

#[allow(dead_code)]
const BLOCK_FULL: &'static str = "\u{2588}";
#[allow(dead_code)]
const BLOCK_UPPER_HALF: &'static str = "\u{2580}";
#[allow(dead_code)]
const BLOCK_LOWER_HALF: &'static str = "\u{2584}";

pub enum Mode {
    Truncate,
    ScaleDownX,
    ScaleDownY,
    ScaleDown,
}

pub struct Chart {
    mode: Mode,
    width: usize,
    height: usize,
    canvas: Canvas,
}

impl Chart {
    pub fn new(width: usize, height: usize, mode: Mode) -> Self {
        let canvas = Canvas::new(width, height);
        Chart { width, height, canvas, mode }
    }

    #[allow(dead_code)]
    pub fn draw_points(&self, points: Vec<Point>) -> String {
        let matrix = PointMatrix::new_from_vec(points);

        match self.mode {
            Mode::Truncate => self.canvas.draw_points(matrix),
            Mode::ScaleDownX => self.canvas.draw_points(transform::scale_x(self, &matrix)),
            Mode::ScaleDownY => self.canvas.draw_points(transform::scale_y(self, &matrix)),
            Mode::ScaleDown => self.canvas.draw_points(transform::scale(self, &matrix)),
        }
    }

    #[allow(dead_code)]
    pub fn draw_points_with_symbol(&self, points: Vec<Point>, symbol: &str) -> String {
        let matrix = PointMatrix::new_from_vec(points);

        match self.mode {
            Mode::Truncate => self.canvas.draw_points_with_symbol(matrix, symbol),
            Mode::ScaleDownX => self.canvas.draw_points_with_symbol(transform::scale_x(self, &matrix), symbol),
            Mode::ScaleDownY => self.canvas.draw_points_with_symbol(transform::scale_y(self, &matrix), symbol),
            Mode::ScaleDown => self.canvas.draw_points_with_symbol(transform::scale(self, &matrix), symbol),
        }
    }

    #[allow(dead_code)]
    pub fn draw_points_with_symbols(&self, points: Vec<Point>, point_symbol: &str, placeholder: &str) -> String {
        let matrix = PointMatrix::new_from_vec(points);

        match self.mode {
            Mode::Truncate => self.canvas.draw_points_with_symbols(matrix, point_symbol, placeholder),
            Mode::ScaleDownX => self.canvas.draw_points_with_symbols(transform::scale_x(self, &matrix), point_symbol, placeholder),
            Mode::ScaleDownY => self.canvas.draw_points_with_symbols(transform::scale_y(self, &matrix), point_symbol, placeholder),
            Mode::ScaleDown => self.canvas.draw_points_with_symbols(transform::scale(self, &matrix), point_symbol, placeholder),
        }
    }

    #[allow(dead_code)]
    pub fn draw_points_with_callback<F>(&self, points: Vec<Point>, draw_callback: F) -> String
        where F: Fn(Option<Point>) -> String {
        let matrix = PointMatrix::new_from_vec(points);

        match self.mode {
            Mode::Truncate => self.canvas.draw_points_with_callback(matrix, &draw_callback),
            Mode::ScaleDownX => self.canvas.draw_points_with_callback(transform::scale_x(self, &matrix), &draw_callback),
            Mode::ScaleDownY => self.canvas.draw_points_with_callback(transform::scale_y(self, &matrix), &draw_callback),
            Mode::ScaleDown => self.canvas.draw_points_with_callback(transform::scale(self, &matrix), &draw_callback),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draw_points_with_symbol_test() {
        let canvas = Chart::new(10, 2, Mode::Truncate);

        assert_eq!(
            ".         \n .        \n",
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
            "😊         \n 😊        \n",
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
            "😊         \n 😊       😊\n",
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
            ".         \n .        \n",
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
            "😊         \n 😊        \n",
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
            "😊_________\n_😊_______😊\n",
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
            "x_________\n_x________\n",
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
    }
}
