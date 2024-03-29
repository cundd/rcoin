extern crate serde;
extern crate serde_json;
extern crate clap;
extern crate term_size;
extern crate chrono;
extern crate rand;

#[macro_use]
mod error;
mod term_style;
mod rate;
mod rate_provider;
mod util;
mod chart;
mod matrix;
#[macro_use]
mod ui;
mod rate_printer;
mod point;
mod signal_handler;

use std::{thread, time};
use clap::{App, Arg, ArgMatches};
use ui::CoordinatePrecision;
use rate_provider::*;

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

fn get_dimension_argument(name: &str, matches: &ArgMatches) -> CoordinatePrecision {
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

fn get_chart_width(matches: &ArgMatches) -> CoordinatePrecision {
    get_dimension_argument("width", matches)
}

fn get_chart_height(matches: &ArgMatches) -> CoordinatePrecision {
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

fn get_provider(matches: &ArgMatches) -> String {
    match matches.value_of("provider") {
        Some(provider) => provider.to_string(),
        None => CoinMarketCap::get_name().to_string(),
    }
}

fn get_value(matches: &ArgMatches) -> Option<f32> {
    match matches.value_of("VALUE") {
        Some(arg) => arg.parse::<f32>().ok(),
        None => None,
    }
}

fn get_currency(matches: &ArgMatches) -> rate::Currency {
    let input = matches.value_of("CURRENCY").unwrap();
    if let Some(c) = rate::Currency::new(input) {
        return c;
    }

    panic!("Currency {} not supported", input)
}

fn get_all_providers() -> String {
    rate_provider::get_all_names()
        .iter().map(|s| s.to_string()).collect::<Vec<String>>()
        .join(", ")
}


fn exit() {
    term_style::cursor::show_cursor();
    ::std::process::exit(1);
}

extern fn received_signal(_: i32) {
    exit();
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
        .arg(Arg::with_name("VALUE")
            .help("Coin value to convert")
            .index(2))
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
        .arg(Arg::with_name("provider")
            .long("provider")
            .help(&format!("Fetch rates from the given provider [{}]", get_all_providers()))
            .takes_value(true))
        .get_matches();


    let mut keyboard_listener = ui::keyboard::KeyboardListener::new();
    keyboard_listener.add_listener('q', |_| {
        return true;
    });
    signal_handler::register(signal_handler::Signal::SIGINT, received_signal);

    let interval = time::Duration::from_millis(get_interval(&matches) / 5);
    let fill = get_chart_point(&matches);
    let space = get_chart_fill(&matches);
    let value = get_value(&matches);
    let provider = get_provider(&matches);
    let currency = get_currency(&matches);

    let chart = chart::Chart::new(
        get_chart_width(&matches),
        get_chart_height(&matches),
        0,
        10,
        get_mode(&matches),
    );

    let mut printer = match rate_printer::RatePrinter::new(chart, value, &provider, &fill, &space, get_history_size(&matches)) {
        Ok(p) => p,
        Err(error) => error!("{}", error),
    };
    let mut run_number = 0;
    let mut error: Option<self::ui::Error> = None;
    term_style::cursor::hide_cursor();

    loop {
        if run_number == 0 || run_number % 5 == 0 {
            if let Err(e) = printer.get_and_print_rates(currency) {
                error = Some(e);
                break;
            }
        }
        let result: Vec<bool> = keyboard_listener.listen();
        if !result.is_empty() && result.into_iter().find(|i| *i == true).is_some() {
            break;
        }
        run_number += 1;

        thread::sleep(interval);
    }

    if let Some(error) = error {
        error!("{}", error);
    }

    exit();
}

