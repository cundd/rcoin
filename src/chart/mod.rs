mod canvas;
mod point;
mod point_matrix;

pub use self::point::Point;
use self::canvas::Canvas;

const BLOCK_FULL: &'static str = "\u{2588}";
const BLOCK_UPPER_HALF: &'static str = "\u{2580}";
const BLOCK_LOWER_HALF: &'static str = "\u{2584}";

pub struct Chart {
    width: usize,
    height: usize,
    canvas: Canvas,
}

impl Chart {
    pub fn new(width: usize, height: usize) -> Self {
        let canvas = Canvas::new(width, height);
        Chart { width, height, canvas }
    }

    #[allow(dead_code)]
    pub fn draw_points(&self, points: Vec<&Point>) -> String {
        self.canvas.draw_points(points)
    }

    #[allow(dead_code)]
    pub fn draw_points_with_symbol(&self, points: Vec<&Point>, symbol: &str) -> String {
        self.canvas.draw_points_with_symbol(points, symbol)
    }

    #[allow(dead_code)]
    pub fn draw_points_with_symbols(&self, points: Vec<&Point>, point_symbol: &str, placeholder: &str) -> String {
        self.canvas.draw_points_with_symbols(points, point_symbol, placeholder)
    }

    #[allow(dead_code)]
    pub fn draw_points_with_callback<F>(&self, points: Vec<&Point>, draw_callback: F) -> String
        where F: Fn(Option<Point>) -> String {
        self.canvas.draw_points_with_callback(points, &draw_callback)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draw_points_with_symbol_test() {
        let canvas = Chart::new(10, 2);

        assert_eq!(
            ".         \n .        \n",
            canvas.draw_points_with_symbol(
                vec![
                    &Point::new(0, 0),
                    &Point::new(1, 1),
                    &Point::new(2, 2),      // Will be clipped
                    &Point::new(10, 20),    // Will be clipped
                    &Point::new(12, 20),    // Will be clipped
                    &Point::new(14, 20),    // Will be clipped
                    &Point::new(11, 20),    // Will be clipped
                    &Point::new(99, 20),    // Will be clipped
                    &Point::new(100, 20),   // Will be clipped
                    &Point::new(101, 20)    // Will be clipped
                ],
                "."
            )
        );

        assert_eq!(
            "😊         \n 😊        \n",
            canvas.draw_points_with_symbol(
                vec![
                    &Point::new(0, 0),
                    &Point::new(1, 1),
                    &Point::new(2, 2),      // Will be clipped
                    &Point::new(10, 20),    // Will be clipped
                    &Point::new(12, 20),    // Will be clipped
                    &Point::new(14, 20),    // Will be clipped
                    &Point::new(11, 20),    // Will be clipped
                    &Point::new(99, 20),    // Will be clipped
                    &Point::new(100, 20),   // Will be clipped
                    &Point::new(101, 20)    // Will be clipped
                ],
                "😊"
            )
        );

        assert_eq!(
            "😊         \n 😊       😊\n",
            canvas.draw_points_with_symbol(
                vec![
                    &Point::new(0, 0),
                    &Point::new(1, 1),
                    &Point::new(9, 1),
                    &Point::new(2, 2),      // Will be clipped
                    &Point::new(10, 20),    // Will be clipped
                    &Point::new(12, 20),    // Will be clipped
                    &Point::new(14, 20),    // Will be clipped
                    &Point::new(11, 20),    // Will be clipped
                    &Point::new(99, 20),    // Will be clipped
                    &Point::new(100, 20),   // Will be clipped
                    &Point::new(101, 20)    // Will be clipped
                ],
                "😊"
            )
        );
    }

    #[test]
    fn draw_points_with_symbols_test() {
        let canvas = Chart::new(10, 2);

        assert_eq!(
            ".         \n .        \n",
            canvas.draw_points_with_symbols(
                vec![
                    &Point::new(0, 0),
                    &Point::new(1, 1),
                    &Point::new(2, 2),      // Will be clipped
                    &Point::new(10, 20),    // Will be clipped
                    &Point::new(12, 20),    // Will be clipped
                    &Point::new(14, 20),    // Will be clipped
                    &Point::new(11, 20),    // Will be clipped
                    &Point::new(99, 20),    // Will be clipped
                    &Point::new(100, 20),   // Will be clipped
                    &Point::new(101, 20)    // Will be clipped
                ],
                ".",
                " "
            )
        );

        assert_eq!(
            "😊         \n 😊        \n",
            canvas.draw_points_with_symbols(
                vec![
                    &Point::new(0, 0),
                    &Point::new(1, 1),
                    &Point::new(2, 2),      // Will be clipped
                    &Point::new(10, 20),    // Will be clipped
                    &Point::new(12, 20),    // Will be clipped
                    &Point::new(14, 20),    // Will be clipped
                    &Point::new(11, 20),    // Will be clipped
                    &Point::new(99, 20),    // Will be clipped
                    &Point::new(100, 20),   // Will be clipped
                    &Point::new(101, 20)    // Will be clipped
                ],
                "😊",
                " "
            )
        );

        assert_eq!(
            "😊_________\n_😊_______😊\n",
            canvas.draw_points_with_symbols(
                vec![
                    &Point::new(0, 0),
                    &Point::new(1, 1),
                    &Point::new(9, 1),
                    &Point::new(2, 2),      // Will be clipped
                    &Point::new(10, 20),    // Will be clipped
                    &Point::new(12, 20),    // Will be clipped
                    &Point::new(14, 20),    // Will be clipped
                    &Point::new(11, 20),    // Will be clipped
                    &Point::new(99, 20),    // Will be clipped
                    &Point::new(100, 20),   // Will be clipped
                    &Point::new(101, 20)    // Will be clipped
                ],
                "😊",
                "_"
            )
        );
    }

    #[test]
    fn draw_points_with_callback_test() {
        let canvas = Chart::new(10, 2);

        assert_eq!(
            "x_________\n_x________\n",
            canvas.draw_points_with_callback(
                vec![
                    &Point::new(0, 0),
                    &Point::new(1, 1),
                    &Point::new(2, 2),      // Will be clipped
                    &Point::new(10, 20),    // Will be clipped
                    &Point::new(12, 20),    // Will be clipped
                    &Point::new(14, 20),    // Will be clipped
                    &Point::new(11, 20),    // Will be clipped
                    &Point::new(99, 20),    // Will be clipped
                    &Point::new(100, 20),   // Will be clipped
                    &Point::new(101, 20)    // Will be clipped
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
