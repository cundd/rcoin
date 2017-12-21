use std::io::Write;
use std::io::stdout;
use chrono::prelude::*;
use util;
use rate;
use rate::RateSeries;
use chart::*;
use rate_provider;
use matrix;

mod trend;

pub struct RatePrinter<'a> {
    fill: &'a str,
    space: &'a str,
    provider_type: &'a str,
    time_series: rate::RateSeries,
    chart: Chart,
    run_number: usize,
}

impl<'a> RatePrinter<'a> {
    pub fn new(chart: Chart, provider_type: &'a str, fill: &'a str, space: &'a str, history_size: Option<usize>) -> Self {
        let time_series = rate::RateSeries::new(
            match history_size {
                Some(history_size) => history_size,
                None => 1 * (chart.width() - chart.y_scala_width),
            }
        );

        RatePrinter {
            space,
            fill,
            provider_type,
            chart,
            time_series,
            run_number: 0,
        }
    }

    pub fn get_and_print_rates(&mut self, currency: rate_provider::Currency) -> Result<rate::Rate, ()> {
        self.run_number += 1;
        match rate_provider::get(self.provider_type, currency) {
            Ok(rate) => {
                let last_rate = self.time_series.last().cloned();
                self.time_series.push(rate.clone());

                println!();
                print!("{}[2J", 27 as char); // Clear the screen
                self.print_header(&rate, &last_rate);
                self.print_chart(&rate, &last_rate);
                self.print_footer(&rate, &last_rate);

                Ok(rate)
            }
            Err(e) => {
                println!("{}", e.to_string());
                Err(())
            }
        }
    }

    fn print_chart(&self, rate: &rate::Rate, last_rate: &Option<rate::Rate>) {
        let conf = configuration::CallbackConfiguration {
            draw_row: |n: usize| util::str_left_pad(&format!("{} |", n), self.chart.y_scala_width - 1, ' ').to_string(),
            draw_point: |point: Option<Point>| self.draw_callback(&rate, last_rate, point),
        };

        print!(
            "{}",
            self.chart.draw_points_with_configuration(
                build_points_from_time_series(&self.time_series),
                &conf,
            )
        );
    }

    fn print_footer(&self, rate: &rate::Rate, last_rate: &Option<rate::Rate>) {
        let now: DateTime<Local> = Local::now();

        let col_1 = now.format("%Y-%m-%d %H:%M:%S%.3f").to_string();
        let col_2 = trend::get_trend_sign(&rate, last_rate, true);
        let col_3 = format!(
            "{} ${} / €{}",
            util::str_pad(&rate.symbol, 5, ' '),
            util::str_pad(&rate.price_usd.to_string(), 10, ' '),
            util::str_pad(&rate.price_eur.to_string(), 10, ' ')
        );
        let col_left_visible_width = col_1.chars().count()
            + 1
            + trend::get_trend_sign(&rate, last_rate, false).chars().count()
            + 1
            + col_3.chars().count()
            + 1;

        let space_left = self.chart.width() as isize - col_left_visible_width as isize;

        let provider_name = rate_provider::get_name(self.provider_type).unwrap_or("");
        let col_4 = format!("[{}]", provider_name);

        if space_left >= (col_4.chars().count() as isize) {
            print!("{} {} {} {}", col_1, col_2, col_3, util::str_left_pad(&format!("[{}]", provider_name), space_left as usize, ' '));
        } else {
            print!("{} {} {}", col_1, col_2, col_3);
        }

        stdout().flush().unwrap();
    }

//    fn print_header(&self, rate: &rate::Rate, last_rate: &Option<rate::Rate>) {
    fn print_header(&self, _: &rate::Rate, _: &Option<rate::Rate>) {
        let matrix = matrix::Matrix::from_slice(&self.time_series.data());


//        println!("Span from {:?} to {:?} / {:?} > {:?}", matrix.y_min(), matrix.y_max(), rate, build_point_from_rate(1, rate));
        println!("Span from {:?} to {:?}", matrix.y_min(), matrix.y_max());
//        return ();
//        let now: DateTime<Local> = Local::now();
//
//        let col_1 = now.format("%Y-%m-%d %H:%M:%S%.3f").to_string();
//        let col_2 = trend::get_trend_sign(&rate, last_rate, true);
//        let col_3 = format!(
//            "{} ${} / €{}",
//            util::str_pad(&rate.symbol, 5, ' '),
//            util::str_pad(&rate.price_usd.to_string(), 10, ' '),
//            util::str_pad(&rate.price_eur.to_string(), 10, ' ')
//        );
//        let col_left_visible_width = col_1.chars().count()
//            + 1
//            + trend::get_trend_sign(&rate, last_rate, false).chars().count()
//            + 1
//            + col_3.chars().count()
//            + 1;
//
//        let space_left = self.chart.width() as isize - col_left_visible_width as isize;
//
//        let provider_name = rate_provider::get_name(self.provider_type).unwrap_or("");
//        let col_4 = format!("[{}]", provider_name);
//
//        if space_left >= (col_4.chars().count() as isize) {
//            print!("{} {} {} {}", col_1, col_2, col_3, util::str_left_pad(&format!("[{}]", provider_name), space_left as usize, ' '));
//        } else {
//            print!("{} {} {}", col_1, col_2, col_3);
//        }
//
//        println!()
////        stdout().flush().unwrap();
    }

    fn draw_callback(&self, current_rate: &rate::Rate, last_rate: &Option<rate::Rate>, point: Option<Point>) -> String {
        match point {
            Some(_) => {
                let trend = trend::get_trend(current_rate, last_rate);
                match trend {
                    -1 => color::red(self.fill),
                    1 => color::green(self.fill),
                    0 => color::dark_gray(self.fill),
                    _ => color::bg_red(self.fill),
                }
            }
            None => self.space.to_string(),
        }
    }
}

fn build_points_from_time_series(time_series: &RateSeries) -> Vec<Point> {
    let mut points: Vec<Point> = vec![];
    let mut len = 0;
    for rate in time_series.data() {
        points.push(build_point_from_rate(len, rate));
        len += 1;
    }

    points
}

#[inline]
fn build_point_from_rate(x: usize, rate: &rate::Rate) -> Point {
    Point::new(x, rate.price_usd.round() as usize)
}
