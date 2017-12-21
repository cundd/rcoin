use matrix::Matrix;
use super::point::Point;
use super::Chart;

fn scale_with_factors(matrix: &Matrix<Point>, factor_x: f32, factor_y: f32) -> Matrix<Point> {
    matrix.map(|p| Point::new(
        (p.x as f32 * factor_x).floor() as usize,
        (p.y as f32 * factor_y).floor() as usize,
    ))
}

#[allow(unused)]
fn get_factor(chart: &Chart, matrix: &Matrix<Point>, max: Option<f32>) -> (f32, f32) {
    (get_factor_x(chart, matrix, max), get_factor_y(chart, matrix, max))
}

fn get_factor_x(chart: &Chart, matrix: &Matrix<Point>, max: Option<f32>) -> f32 {
    if matrix.is_empty() {
        return 1.0;
    }
    let result = (chart.width() - chart.x_scala_width) as f32 /
        (1 + matrix.x_max().unwrap() - matrix.x_min().unwrap()) as f32;
    //   ^__ include the max point

    match max {
        None => result,
        Some(max) => if result < max {
            result
        } else {
            max
        }
    }
}

fn get_factor_y(chart: &Chart, matrix: &Matrix<Point>, max: Option<f32>) -> f32 {
    if matrix.is_empty() {
        return 1.0;
    }
    let result = (chart.height() - chart.y_scala_width) as f32 /
        (1 + matrix.y_max().unwrap() - matrix.y_min().unwrap()) as f32;
    //   ^__ include the max point

    match max {
        None => result,
        Some(max) => if result < max {
            result
        } else {
            max
        }
    }
}

pub fn scale(chart: &Chart, matrix: &Matrix<Point>) -> Matrix<Point> {
    scale_with_factors(matrix, get_factor_x(chart, matrix, None), get_factor_y(chart, matrix, None))
}

pub fn scale_x(chart: &Chart, matrix: &Matrix<Point>) -> Matrix<Point> {
    scale_with_factors(matrix, get_factor_x(chart, matrix, None), 1.0)
}

pub fn scale_y(chart: &Chart, matrix: &Matrix<Point>) -> Matrix<Point> {
    scale_with_factors(matrix, 1.0, get_factor_y(chart, matrix, None))
}

pub fn scale_down(chart: &Chart, matrix: &Matrix<Point>) -> Matrix<Point> {
    scale_with_factors(matrix, get_factor_x(chart, matrix, Some(1.0)), get_factor_y(chart, matrix, Some(1.0)))
}

pub fn scale_down_x(chart: &Chart, matrix: &Matrix<Point>) -> Matrix<Point> {
    scale_with_factors(matrix, get_factor_x(chart, matrix, Some(1.0)), 1.0)
}

pub fn scale_down_y(chart: &Chart, matrix: &Matrix<Point>) -> Matrix<Point> {
    scale_with_factors(matrix, 1.0, get_factor_y(chart, matrix, Some(1.0)))
}


