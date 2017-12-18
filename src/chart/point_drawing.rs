use super::point::Point;
use super::configuration::Configuration;

pub trait PointDrawing {
    #[allow(unused)]
    fn draw_points(&self, points: Vec<Point>) -> String;
    #[allow(unused)]
    fn draw_points_with_configuration(&self, points: Vec<Point>, conf: &Configuration) -> String;
    #[allow(unused)]
    fn draw_points_with_symbol(&self, points: Vec<Point>, symbol: &str) -> String;
    #[allow(unused)]
    fn draw_points_with_symbols(&self, points: Vec<Point>, point_symbol: &str, placeholder: &str) -> String;
    #[allow(unused)]
    fn draw_points_with_callback<F>(&self, points: Vec<Point>, draw_callback: F) -> String
        where F: Fn(Option<Point>) -> String;
}
