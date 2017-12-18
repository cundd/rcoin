use super::Point;

pub trait Configuration {
    fn draw_row(&self, row_number: usize) -> String;

    fn draw_point(&self, point: Option<Point>) -> String;
}

pub struct CallbackConfiguration<R, P>
    where R: Fn(usize) -> String,
          P: Fn(Option<Point>) -> String {
    pub draw_row: R,
    pub draw_point: P,
}

impl<R, P> CallbackConfiguration<R, P>
    where R: Fn(usize) -> String,
          P: Fn(Option<Point>) -> String {
    pub fn draw_row(&self, row_number: usize) -> String {
        let callback = &self.draw_row;
        callback(row_number)
    }

    pub fn draw_point(&self, point: Option<Point>) -> String {
        let callback = &self.draw_point;
        callback(point)
    }
}

impl<R, P> Configuration for CallbackConfiguration<R, P>
    where R: Fn(usize) -> String,
          P: Fn(Option<Point>) -> String {
    fn draw_row(&self, row_number: usize) -> String {
        CallbackConfiguration::draw_row(self, row_number)
    }

    fn draw_point(&self, point: Option<Point>) -> String {
        CallbackConfiguration::draw_point(self, point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_test() {
        let config = CallbackConfiguration {
            draw_row: |r| format!("{}", r),
            draw_point: |p| match p {
                Some(p) => format!("{:?}", p),
                None => "None".to_string()
            },
        };
        assert_eq!("102", config.draw_row(102));
        assert_eq!("None", config.draw_point(None));
    }
}
