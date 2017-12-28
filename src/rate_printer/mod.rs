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
        let time_series = build_time_series(&chart, history_size);
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
                panic!(e);
                // println!("{}", e.to_string());
                // Err(())
            }
        }
    }

    fn draw_row(&self, row: Option<&matrix::Row<rate::Rate>>, row_number: usize) -> String {
        let header = match row {
            Some(row) => {
                let (_, rate) = row.iter().next().expect(&format!("No items found in row at {}", row_number));

                format!("{} |", rate::Rate::price_to_coordinate(rate.price_usd))
            }
            None => "|".to_string(),
        };

        util::str_left_pad(
            &header,
            self.chart.y_scala_width,
            ' ',
        ).to_string()
    }

    fn print_chart(&self, rate: &rate::Rate, last_rate: &Option<rate::Rate>) {
        let conf = configuration::CallbackConfiguration::new(
            |row: Option<&matrix::Row<rate::Rate>>, row_number: usize| self.draw_row(row, row_number),
            |point: Option<rate::Rate>| self.draw_callback(&rate, last_rate, point),
        );

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

        let footer_complete = if space_left >= (col_4.chars().count() as isize) {
            format!("{} {} {} {}", col_1, col_2, col_3, util::str_left_pad(&format!("[{}]", provider_name), space_left as usize, ' '))
        } else {
            format!("{} {} {}", col_1, col_2, col_3)
        };

        print!("{}", color::reverse(&footer_complete));

        stdout().flush().unwrap();
    }

    fn print_header(&self, rate: &rate::Rate, _: &Option<rate::Rate>) {
        #[cfg(debug_assertions)]
            {
                let matrix = matrix::Matrix::from_slice(&self.time_series.data());

                println!("{:?}", self.time_series);
                println!("Span from {:?} to {:?}", matrix.y_min(), matrix.y_max());
                println!("{:#?}", matrix);
            }
    }

    fn draw_callback<T: matrix::PointTrait>(&self, current_rate: &rate::Rate, last_rate: &Option<rate::Rate>, point: Option<T>) -> String {
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


fn build_points_from_time_series(time_series: &RateSeries) -> Vec<rate::Rate> {
    let mut points: Vec<rate::Rate> = vec![];
    let mut len = 0;
    for rate in time_series.data() {
        points.push(matrix::PointTrait::with_x(rate, len));
        len += 1;
    }

    points
}

fn build_time_series(chart: &Chart, history_size: Option<usize>) -> RateSeries {
    let chart_width = chart.width();
    let prepared_history_size = match history_size {
        Some(history_size) => {
            if history_size > 0 {
                history_size
            } else {
                panic!("History size must be bigger than zero")
            }
        }
        None => {
            if chart_width <= chart.y_scala_width {
                error!("Chart width must be bigger than {}", chart.y_scala_width)
            } else {
                chart_width - chart.y_scala_width
            }
        }
    };

    rate::RateSeries::new(prepared_history_size)
}
