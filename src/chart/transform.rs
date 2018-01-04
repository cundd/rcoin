use matrix::Matrix;
use matrix::PointTrait;
//use super::point::Point;
use super::Chart;
use ui::CoordinatePrecision;

fn scale_with_factors<T: PointTrait>(matrix: &Matrix<T>, factor_x: f32, factor_y: f32) -> Matrix<T> {
    matrix.map(|p| p.with_x_y(
        (p.x() as f32 * factor_x).floor() as CoordinatePrecision,
        (p.y() as f32 * factor_y).floor() as CoordinatePrecision,
    ))
}

#[allow(unused)]
fn get_factor<T: PointTrait>(chart: &Chart, matrix: &Matrix<T>, max: Option<f32>) -> (f32, f32) {
    (get_factor_x(chart, matrix, max), get_factor_y(chart, matrix, max))
}

fn get_factor_x<T: PointTrait>(chart: &Chart, matrix: &Matrix<T>, max: Option<f32>) -> f32 {
    if matrix.is_empty() {
        return 1.0;
    }

    // The scala for the y-axis takes away from the available width
    let result = (chart.width() - chart.y_scala_width) as f32 /
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

fn get_factor_y<T: PointTrait>(chart: &Chart, matrix: &Matrix<T>, max: Option<f32>) -> f32 {
    if matrix.is_empty() {
        return 1.0;
    }

    // The scala for the x-axis takes away from the available height
    let result = (chart.height() - chart.x_scala_height) as f32 /
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

pub fn scale<T: PointTrait>(chart: &Chart, matrix: &Matrix<T>) -> Matrix<T> {
    scale_with_factors(matrix, get_factor_x(chart, matrix, None), get_factor_y(chart, matrix, None))
}

pub fn scale_x<T: PointTrait>(chart: &Chart, matrix: &Matrix<T>) -> Matrix<T> {
    scale_with_factors(matrix, get_factor_x(chart, matrix, None), 1.0)
}

pub fn scale_y<T: PointTrait>(chart: &Chart, matrix: &Matrix<T>) -> Matrix<T> {
    scale_with_factors(matrix, 1.0, get_factor_y(chart, matrix, None))
}

pub fn scale_down<T: PointTrait>(chart: &Chart, matrix: &Matrix<T>) -> Matrix<T> {
    scale_with_factors(matrix, get_factor_x(chart, matrix, Some(1.0)), get_factor_y(chart, matrix, Some(1.0)))
}

pub fn scale_down_x<T: PointTrait>(chart: &Chart, matrix: &Matrix<T>) -> Matrix<T> {
    scale_with_factors(matrix, get_factor_x(chart, matrix, Some(1.0)), 1.0)
}

pub fn scale_down_y<T: PointTrait>(chart: &Chart, matrix: &Matrix<T>) -> Matrix<T> {
    scale_with_factors(matrix, 1.0, get_factor_y(chart, matrix, Some(1.0)))
}


