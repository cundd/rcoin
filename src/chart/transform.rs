use super::point::Point;
use super::point_matrix::PointMatrix;
use super::Chart;

fn scale_with_factors(matrix: &PointMatrix, factor_x: f32, factor_y: f32) -> PointMatrix {
    matrix.map(|p| Point::new(
        (p.x as f32 * factor_x).round() as usize,
        (p.y as f32 * factor_y).round() as usize,
    ))
}

#[allow(unused)]
fn get_factor(chart: &Chart, matrix: &PointMatrix, max: Option<f32>) -> (f32, f32) {
    (get_factor_x(chart, matrix, max), get_factor_y(chart, matrix, max))
}

fn get_factor_x(chart: &Chart, matrix: &PointMatrix, max: Option<f32>) -> f32 {
    if matrix.is_empty() {
        return 1.0;
    }
    let result = chart.width as f32 / (matrix.x_max().unwrap() as f32 - matrix.x_min().unwrap() as f32);

    match max {
        None => result,
        Some(max) => if result < max {
            result
        } else {
            max
        }
    }
}

fn get_factor_y(chart: &Chart, matrix: &PointMatrix, max: Option<f32>) -> f32 {
    if matrix.is_empty() {
        return 1.0;
    }
    let result = chart.height as f32 / (matrix.y_max().unwrap() as f32 - matrix.y_min().unwrap() as f32);

    match max {
        None => result,
        Some(max) => if result < max {
            result
        } else {
            max
        }
    }
}

pub fn scale(chart: &Chart, matrix: &PointMatrix) -> PointMatrix {
    scale_with_factors(matrix, get_factor_x(chart, matrix, None), get_factor_y(chart, matrix, None))
}

pub fn scale_x(chart: &Chart, matrix: &PointMatrix) -> PointMatrix {
    scale_with_factors(matrix, get_factor_x(chart, matrix, None), 1.0)
}

pub fn scale_y(chart: &Chart, matrix: &PointMatrix) -> PointMatrix {
    scale_with_factors(matrix, 1.0, get_factor_y(chart, matrix, None))
}

pub fn scale_down(chart: &Chart, matrix: &PointMatrix) -> PointMatrix {
    scale_with_factors(matrix, get_factor_x(chart, matrix, Some(1.0)), get_factor_y(chart, matrix, Some(1.0)))
}

pub fn scale_down_x(chart: &Chart, matrix: &PointMatrix) -> PointMatrix {
    scale_with_factors(matrix, get_factor_x(chart, matrix, Some(1.0)), 1.0)
}

pub fn scale_down_y(chart: &Chart, matrix: &PointMatrix) -> PointMatrix {
    scale_with_factors(matrix, 1.0, get_factor_y(chart, matrix, Some(1.0)))
}


