use chrono::prelude::*;
use util;
use rate;
use rate::RateSeries;
use chart::*;
use term_style::style as color;
use rate_provider;
use matrix;
use ui::Error;
use ui::Screen;
use ui::Size;
use ui::CoordinatePrecision;
use ui::PixelSequenceTrait;
use ui::PixelSequence;
use ui::medium::*;
use point::Point;

mod trend;

pub struct RatePrinter<'a> {
    value: Option<f32>,
    fill: &'a str,
    space: &'a str,
    provider: &'a str,
    time_series: rate::RateSeries,
    chart: Chart,
    run_number: usize,
    screen: Screen<Terminal>,
}

impl<'a> RatePrinter<'a> {
    pub fn new(chart: Chart, value: Option<f32>, provider: &'a str, fill: &'a str, space: &'a str, history_size: Option<usize>) -> Result<Self, Error> {
        let time_series = build_time_series(&chart, history_size);
        let screen = Screen::default()?;
        Ok(RatePrinter {
            space,
            fill,
            value,
            provider,
            chart,
            time_series,
            screen,
            run_number: 0,
        })
    }

    pub fn get_and_print_rates(&mut self, currency: rate::Currency) -> Result<rate::Rate, Error> {
        self.run_number += 1;
        match rate_provider::get(self.provider, currency) {
            Ok(rate) => {
                let last_rate = self.time_series.last().cloned();
                self.time_series.push(rate.clone());

                let output = format!(
                    "{}{}{}",
                    self.get_header(&rate, &last_rate),
                    self.get_chart(&rate, &last_rate),
                    self.get_footer(&rate, &last_rate),
                );

                let screen = self.get_screen();
                screen.draw_multi_line_text(&Point::new(0, 0), &output)?;
                screen.flush()?;

                Ok(rate)
            }
            Err(e) => Err(ui_error!(Misc, "{}", e.to_string()))
        }
    }

    fn draw_row(&self, row: Option<&matrix::Row<rate::Rate>>, row_number: CoordinatePrecision) -> String {
        let header = match row {
            Some(row) => {
                let (_, rate) = row.iter().next().expect(&format!("No items found in row at {}", row_number));

                if rate.currency.scale() > 1.0 {
                    format!("{:.*} |", 2, rate.price_usd)
                } else {
                    format!("{} |", rate.price_usd.round())
                }
            }
            None => "|".to_string(),
        };

        util::str_left_pad(
            &header,
            self.chart.y_scala_width as usize,
            ' ',
        ).to_string()
    }

    fn get_chart(&self, rate: &rate::Rate, last_rate: &Option<rate::Rate>) -> String {
        let conf = configuration::CallbackConfiguration::new(
            |row: Option<&matrix::Row<rate::Rate>>, row_number: CoordinatePrecision| self.draw_row(row, row_number),
            |point: Option<rate::Rate>| self.draw_callback(&rate, last_rate, point),
        );

        self.chart.draw_points_with_configuration(
            build_points_from_time_series(&self.time_series),
            &conf,
        )
    }

    fn get_footer(&self, rate: &rate::Rate, last_rate: &Option<rate::Rate>) -> String {
        let now: DateTime<Local> = Local::now();

        let col_1 = now.format("%Y-%m-%d %H:%M:%S%.3f").to_string();
        let col_2 = trend::get_trend_sign(&rate, last_rate, true);
        let col_3 = format!(
            "{} ${} / €{}",
            util::str_pad(&rate.currency.symbol(), 5, ' '),
            util::str_pad(&rate.price_usd.to_string(), 10, ' '),
            util::str_pad(&rate.price_eur.to_string(), 10, ' ')
        );

        let mut footer = format!("{} {} {}", col_1, col_2, col_3);

        let col_4 = match self.value {
            Some(value) => {
                format!(
                    "| {} ≈ €{}",
                    value.to_string(),
                    util::str_pad(&format!("{}", value * rate.price_eur), 10, ' '),
                )
            }
            None => "".to_string(),
        };

        let space_left = self.chart.width() as isize - PixelSequence::from_str(&footer).len() as isize;
        if space_left >= (col_4.chars().count() as isize) {
            footer.push_str(&col_4);
        }

        let space_left = space_left - 1 - col_4.chars().count() as isize;
        let provider_name = rate_provider::get_name(self.provider).unwrap_or("");
        let col_5 = format!("[{}]", provider_name);

        if space_left >= (col_5.chars().count() as isize) {
            footer.push_str(&util::str_left_pad(&format!("[{}]", provider_name), space_left as usize, ' '))
        }

        footer
    }

    fn get_header(&self, _: &rate::Rate, _: &Option<rate::Rate>) -> String {
        #[cfg(debug_assertions)]
            {
//                let matrix = matrix::Matrix::from_slice(&self.time_series.data());
//                println!("Span from {:?} to {:?}", matrix.y_min(), matrix.y_max());
//                print!("{}[2J", 27 as char); // Clear the screen
            }

        "".to_string()
    }

    fn draw_callback<P: matrix::PointTrait>(&self, current_rate: &rate::Rate, last_rate: &Option<rate::Rate>, point: Option<P>) -> String {
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

    /// Return the `Screen` for this run
    ///
    /// If the terminal size did not change since the previous run, the last `Screen` will be reused
    fn get_screen(&mut self) -> &mut Screen<Terminal> {
        if self.screen.size() == Size::auto().unwrap() {
            return &mut self.screen;
        }
        self.screen = Screen::default().unwrap();

        &mut self.screen
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
    let prepared_history_size: usize = match history_size {
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
                (chart_width - chart.y_scala_width) as usize
            }
        }
    };

    rate::RateSeries::new(prepared_history_size)
}
