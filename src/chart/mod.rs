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
    pub fn draw_points_with_callback<F>(&self, points: Vec<&Point>, draw_callback: F) -> String
        where F: Fn(Option<Point>) -> String {
        self.canvas.draw_points_with_callback(points, &draw_callback)
    }
}
