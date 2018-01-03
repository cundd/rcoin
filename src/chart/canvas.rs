use matrix::*;
use super::configuration::*;
use super::padding::Padding;


pub struct Canvas {
    width: usize,
    height: usize,
    padding: Padding,
}

impl Canvas {
    pub fn new(width: usize, height: usize, padding: Padding) -> Self {
        Canvas { width, height, padding }
    }

    pub fn draw_points<T: PointTrait>(&self, matrix: Matrix<T>) -> String {
        self.draw_points_with_callback(matrix, |point: Option<T>| {
            match point {
                Some(_) => super::BLOCK_FULL.to_string(),
                None => " ".to_string(),
            }
        })
    }

    pub fn draw_points_with_symbol<T: PointTrait>(&self, matrix: Matrix<T>, symbol: &str) -> String {
        self.draw_points_with_callback(matrix, |point: Option<T>| {
            match point {
                Some(_) => symbol.to_string(),
                None => " ".to_string(),
            }
        })
    }

    pub fn draw_points_with_symbols<T: PointTrait>(&self, matrix: Matrix<T>, point_symbol: &str, placeholder: &str) -> String {
        self.draw_points_with_callback(matrix, |point: Option<T>| {
            match point {
                Some(_) => point_symbol.to_string(),
                None => placeholder.to_string(),
            }
        })
    }

    pub fn draw_points_with_configuration<T: PointTrait>(&self, matrix: Matrix<T>, conf: &Configuration<T>) -> String {
        if matrix.is_empty() {
            return "".to_string();
        }
        let mut buffer = String::with_capacity(self.width * self.height);
        let y_min = matrix.y_min().unwrap();

        let y_start = if y_min < self.padding.bottom { 0 } else { y_min - self.padding.bottom };
        let y_end = y_start + self.height + self.padding.top;

        for row_number in (y_start..y_end).rev() {
            let row = matrix.get_row(row_number);
            buffer.push_str(&conf.draw_row(row, row_number));
            buffer.push_str(&self.draw_row_with_configuration(row_number, &matrix, conf));
            buffer.push('\n');
        }
        buffer
    }

    pub fn draw_points_with_callback<F, T: PointTrait>(&self, matrix: Matrix<T>, draw_callback: F) -> String
        where F: Fn(Option<T>) -> String {
        let conf = super::configuration::CallbackConfiguration::new(
            |_: Option<&Row<T>>, _: usize| "".to_string(),
            draw_callback,
        );
        self.draw_points_with_configuration(matrix, &conf)
    }

    fn draw_row_with_configuration<T: PointTrait>(&self, row_number: usize, matrix: &Matrix<T>, conf: &Configuration<T>) -> String {
        let mut buffer = String::with_capacity(self.width);
        let x_min = matrix.x_min().unwrap();
        let x_start = if x_min < self.padding.left { 0 } else { x_min - self.padding.bottom };
        let x_end = x_start + self.width + self.padding.right;

        for column in x_start..x_end {
            buffer.push_str(&conf.draw_point(matrix.get(row_number, column)));
        }

        buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use point::Point;

    #[test]
    fn draw_points_with_symbol_test() {
        let canvas = Canvas::new(10, 2, Padding::empty());

        assert_eq!(
            " .        \n.         \n",
            canvas.draw_points_with_symbol(
                Matrix::from_vec(vec![
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
            )
        );

        assert_eq!(
            " 😊        \n😊         \n",
            canvas.draw_points_with_symbol(
                Matrix::from_vec(vec![
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
            )
        );

        assert_eq!(
            " 😊       😊\n😊         \n",
            canvas.draw_points_with_symbol(
                Matrix::from_vec(vec![
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
            )
        );
    }

    #[test]
    fn draw_points_with_symbols_test() {
        let canvas = Canvas::new(10, 2, Padding::empty());

        assert_eq!(
            " .        \n.         \n",
            canvas.draw_points_with_symbols(
                Matrix::from_vec(vec![
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
                " ",
            )
        );

        assert_eq!(
            " 😊        \n😊         \n",
            canvas.draw_points_with_symbols(
                Matrix::from_vec(vec![
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
                " ",
            )
        );

        assert_eq!(
            "_😊_______😊\n😊_________\n",
            canvas.draw_points_with_symbols(
                Matrix::from_vec(vec![
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
                "_",
            )
        );
    }

    #[test]
    fn draw_points_with_callback_test() {
        let canvas = Canvas::new(10, 2, Padding::empty());

        assert_eq!(
            "_x________\nx_________\n",
            canvas.draw_points_with_callback(
                Matrix::from_vec(vec![
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
                },
            )
        );
    }
}
