use chrono::prelude::*;
use util;
use rate;
use rate::RateSeries;
use chart::*;
use rate_provider;

mod trend;

pub struct RatePrinter<'a> {
    fill: &'a str,
    space: &'a str,
    provider_type: &'a str,
    time_series: rate::RateSeries,
    chart: Chart,
}

impl<'a> RatePrinter<'a> {
    pub fn new(chart: Chart, provider_type: &'a str, fill: &'a str, space: &'a str) -> Self {
        let time_series = rate::RateSeries::new(1 * chart.width());

        RatePrinter {
            space,
            fill,
            provider_type,
            chart,
            time_series,
        }
    }

    pub fn get_and_print_rates(&mut self) -> Result<rate::Rate, ()> {
        match rate_provider::get(self.provider_type) {
            Ok(rate) => {
                let now: DateTime<Local> = Local::now();
                let ts_clone = self.time_series.clone();
                let last_rate = ts_clone.last();
                let trend = trend::get_trend_sign(&rate, last_rate, true);
                self.time_series.push(rate.clone());

                print!("{}[2J", 27 as char); // Clear the screen
                print!(
                    "{}",
                    self.chart.draw_points_with_callback(
                        build_points_from_time_series(&self.time_series),
                        |point: Option<Point>| self.draw_callback(&rate, last_rate, point)
                    )
                );

                println!(
                    "{}  {}  {} ${} / €{}",
                    now.format("%Y-%m-%d %H:%M:%S"),
                    trend,
                    util::str_pad(&rate.symbol, 5, ' '),
                    util::str_pad(&rate.price_usd.to_string(), 10, ' '),
                    util::str_pad(&rate.price_eur.to_string(), 10, ' ')
                );



                Ok(rate)
            }
            Err(e) => {
                println!("{}", e.to_string());
                Err(())
            }
        }
    }

    fn draw_callback(&self, current_rate: &rate::Rate, last_rate: Option<&rate::Rate>, point: Option<Point>) -> String {
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

    for rate in time_series.data() {
        let len = points.len();
        points.push(Point::new(len, rate.price_eur as usize));
    }

    points
}
