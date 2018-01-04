use std::collections::BTreeMap;
use super::PointTrait;
use ui::CoordinatePrecision;

type Row<I: PointTrait> = BTreeMap<CoordinatePrecision, I>;

pub trait Configuration<T: PointTrait> {
    fn draw_row(&self, row: Option<&Row<T>>, row_number: CoordinatePrecision) -> String;

    fn draw_point(&self, point: Option<T>) -> String;
}

pub struct CallbackConfiguration<R, P, T: PointTrait>
    where
        R: Fn(Option<&Row<T>>, CoordinatePrecision) -> String,
        P: Fn(Option<T>) -> String {
    pub draw_row: R,
    pub draw_point: P,
    _use_t: Option<T>,
}

impl<R, P, T: PointTrait> CallbackConfiguration<R, P, T>
    where
        R: Fn(Option<&Row<T>>, CoordinatePrecision) -> String,
        P: Fn(Option<T>) -> String
{
    pub fn new(draw_row: R, draw_point: P) -> Self {
        CallbackConfiguration {
            draw_row,
            draw_point,
            _use_t: None,
        }
    }
}

impl<R, P, T: PointTrait> CallbackConfiguration<R, P, T>
    where R: Fn(Option<&Row<T>>, CoordinatePrecision) -> String,
          P: Fn(Option<T>) -> String {
    pub fn draw_row(&self, row: Option<&Row<T>>, row_number: CoordinatePrecision) -> String {
        let callback = &self.draw_row;
        callback(row, row_number)
    }

    pub fn draw_point(&self, point: Option<T>) -> String {
        let callback = &self.draw_point;
        callback(point)
    }
}

impl<R, P, T: PointTrait> Configuration<T> for CallbackConfiguration<R, P, T>
    where R: Fn(Option<&Row<T>>, CoordinatePrecision) -> String,
          P: Fn(Option<T>) -> String {
    fn draw_row(&self, row: Option<&Row<T>>, row_number: CoordinatePrecision) -> String {
        CallbackConfiguration::draw_row(self, row, row_number)
    }

    fn draw_point(&self, point: Option<T>) -> String {
        CallbackConfiguration::draw_point(self, point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    struct Point {}

    impl PointTrait for Point {
        fn x(&self) -> CoordinatePrecision { 10 }
        fn y(&self) -> CoordinatePrecision { 20 }
        fn with_x(&self, _: CoordinatePrecision) -> Self { unimplemented!(); }
        fn with_y(&self, _: CoordinatePrecision) -> Self { unimplemented!(); }
        fn with_x_y(&self, _: CoordinatePrecision, _: CoordinatePrecision) -> Self { unimplemented!(); }
    }

    #[test]
    fn call_test() {
        let config = CallbackConfiguration {
            draw_row: |_, r| format!("{}", r),
            draw_point: |p: Option<Point>| match p {
                Some(p) => format!("{}x{}", p.x(), p.y()),
                None => "None".to_string()
            },
            _use_t: None,
        };

        let mut row = Row::new();
        row.insert(102, Point {});
        assert_eq!("102", config.draw_row(Some(&row), 102));
        assert_eq!("None", config.draw_point(None));
        assert_eq!("10x20", config.draw_point(Some(Point {})));
    }
}
