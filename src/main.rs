#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate clap;
extern crate term_size;
extern crate chrono;

#[macro_use]
mod error;
mod term_style;
mod rate;
mod rate_provider;
mod util;
mod chart;
mod matrix;
mod rate_printer;
mod ui;
mod point;

use std::{thread, time};
use clap::{App, Arg, ArgMatches};
use ui::Screen;

fn get_mode(matches: &ArgMatches) -> chart::Mode {
    match matches.value_of("mode") {
        Some(mode_arg) => {
            match chart::Mode::from_str(&mode_arg) {
                Ok(mode) => mode,
                Err(_) => error!("Invalid mode '{}' given", mode_arg),
            }
        }
        None => chart::Mode::ScaleDown,
    }
}

fn get_dimension_argument(name: &str, matches: &ArgMatches) -> usize {
    let default = 0;
    match matches.value_of(name) {
        Some(arg) => {
            match arg.parse() {
                Ok(v) => v,
                Err(_) => error!("Argument '{}' must be a valid positive integer", name)
            }
        }
        None => default,
    }
}

fn get_chart_width(matches: &ArgMatches) -> usize {
    get_dimension_argument("width", matches)
}

fn get_chart_height(matches: &ArgMatches) -> usize {
    get_dimension_argument("height", matches)
}

fn get_history_size(matches: &ArgMatches) -> Option<usize> {
    match matches.value_of("history-size") {
        Some(arg) => {
            match arg.parse::<usize>() {
                Ok(h) => {
                    if h == 0 {
                        error!("Argument 'history-size' must not be smaller than 1")
                    }
                    Some(h)
                }
                Err(_) => error!("Argument 'history-size' must be a valid positive integer and bigger than 0"),
            }
        }
        None => None,
    }
}

fn get_interval(matches: &ArgMatches) -> u64 {
    let default: u64 = 1_000;
    match matches.value_of("interval") {
        Some(arg) => {
            let interval_int = arg.parse::<u64>();
            if let Ok(interval_int) = interval_int {
                return 1_000 * interval_int;
            };
            match arg.parse::<f64>() {
                Ok(interval_float) => {
                    (1_000.0 * interval_float) as u64
                }
                Err(_) => default,
            }
        }
        None => default,
    }
}

fn get_chart_point(matches: &ArgMatches) -> String {
    match matches.value_of("point") {
        Some(point) => point.to_string(),
        None => chart::BLOCK_FULL.to_string(),
    }
}

fn get_chart_fill(matches: &ArgMatches) -> String {
    match matches.value_of("fill") {
        Some(fill) => fill.to_string(),
        None => " ".to_string(),
    }
}

fn get_provider_type(matches: &ArgMatches) -> String {
    match matches.value_of("provider_type") {
        Some(provider_type) => provider_type.to_string(),
        None => "CoinDesk".to_string(),
    }
}

fn get_currency(matches: &ArgMatches) -> rate::Currency {
    let input = matches.value_of("CURRENCY").unwrap();
    if let Some(c) = rate::Currency::new(input) {
        return c;
    }

    panic!("Currency {} not supported", input)
}

fn get_all_provider_types() -> String {
    rate_provider::get_all_names()
        .iter().map(|s| s.to_string()).collect::<Vec<String>>()
        .join(", ")
}

fn main() {
    let matches = App::new("rcoin")
        .version("1.0")
        .author("Daniel Corn <info@cundd.net>")
        .about("Watch crypto-currency prices")
        .arg(Arg::with_name("CURRENCY")
            .help("Sets the currency to monitor")
            .required(true)
            .index(1))
        .arg(Arg::with_name("mode")
            .long("mode")
            .short("m")
            .help("Sets the chart's display mode")
            .takes_value(true))
        .arg(Arg::with_name("history-size")
            .long("history-size")
            .short("s")
            .help("Sets the size of the history")
            .takes_value(true))
        .arg(Arg::with_name("width")
            .long("width")
            .short("w")
            .help("Sets the chart's width")
            .takes_value(true))
        .arg(Arg::with_name("height")
            .long("height")
            .short("h")
            .help("Sets the chart's height")
            .takes_value(true))
        .arg(Arg::with_name("point")
            .long("point")
            .help("Sets the chart's point character")
            .takes_value(true))
        .arg(Arg::with_name("fill")
            .long("fill")
            .help("Sets the chart's fill character")
            .takes_value(true))
        .arg(Arg::with_name("interval")
            .long("interval")
            .short("i")
            .help("Sets the interval between requests (in seconds)")
            .takes_value(true))
        .arg(Arg::with_name("provider_type")
            .long("provider-type")
            .help(&format!("Fetch rates from the given provider [{}]", get_all_provider_types()))
            .takes_value(true))
        .get_matches();

    let interval_seconds = get_interval(&matches);
    let fill = get_chart_point(&matches);
    let space = get_chart_fill(&matches);
    let provider_type = get_provider_type(&matches);
    let currency = get_currency(&matches);

    let chart = chart::Chart::new(
        get_chart_width(&matches),
        get_chart_height(&matches),
        2,//1,
        7,
        get_mode(&matches),
    );

    let screen = Screen::default().unwrap();
    let mut printer = rate_printer::RatePrinter::new(screen, chart, &provider_type, &fill, &space, get_history_size(&matches));
    loop {
        if printer.get_and_print_rates(currency).is_err() {
            break;
        }

        let interval = time::Duration::from_millis(interval_seconds);
        thread::sleep(interval);
    }

    term_style::cursor::show_cursor();
}

