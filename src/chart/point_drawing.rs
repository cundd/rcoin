use super::PointTrait;
use super::configuration::Configuration;

#[allow(unused)]
pub trait PointDrawing {
    fn draw_points<T: PointTrait>(&self, points: Vec<T>) -> String;
    fn draw_points_with_configuration<T: PointTrait>(&self, points: Vec<T>, conf: &Configuration<T>) -> String;
    fn draw_points_with_symbol<T: PointTrait>(&self, points: Vec<T>, symbol: &str) -> String;
    fn draw_points_with_symbols<T: PointTrait>(&self, points: Vec<T>, point_symbol: &str, placeholder: &str) -> String;
    fn draw_points_with_callback<F, T: PointTrait>(&self, points: Vec<T>, draw_callback: F) -> String
        where F: Fn(Option<T>) -> String;
}
