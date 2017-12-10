use super::point::Point;
use super::point_matrix::PointMatrix;
use super::point_matrix::PointMatrixTrait;
use super::Chart;

fn scale_with_factors(matrix: &PointMatrix, factor_x: f32, factor_y: f32) -> PointMatrix {
    matrix.map(|p| Point::new(
        (p.x as f32 * factor_x).round() as usize,
        (p.y as f32 * factor_y).round() as usize,
    ))
}

fn get_factor(chart: &Chart, matrix: &PointMatrixTrait) -> (f32, f32) {
    (get_factor_x(chart, matrix), get_factor_y(chart, matrix))
}

fn get_factor_x(chart: &Chart, matrix: &PointMatrixTrait) -> f32 {
    if matrix.is_empty() {
        return 1.0;
    }
    chart.width as f32 / (matrix.x_max().unwrap() as f32 - matrix.x_min().unwrap() as f32)
}

fn get_factor_y(chart: &Chart, matrix: &PointMatrixTrait) -> f32 {
    if matrix.is_empty() {
        return 1.0;
    }
    chart.height as f32 / (matrix.y_max().unwrap() as f32 - matrix.y_min().unwrap() as f32)
}


pub fn scale(chart: &Chart, matrix: &PointMatrix) -> PointMatrix {
    scale_with_factors(matrix, get_factor_x(chart, matrix), get_factor_y(chart, matrix))
}

pub fn scale_x(chart: &Chart, matrix: &PointMatrix) -> PointMatrix {
    scale_with_factors(matrix, get_factor_x(chart, matrix), 1.0)
}

pub fn scale_y(chart: &Chart, matrix: &PointMatrix) -> PointMatrix {
    scale_with_factors(matrix, 1.0, get_factor_y(chart, matrix))
}

