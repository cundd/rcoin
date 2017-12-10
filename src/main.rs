#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
//extern crate curl;

mod rate;
mod rate_provider;
mod util;
mod chart;

use std::{thread, time};
use chart::Point;

fn get_and_print_rates(last_rate_option: Result<rate::Rate, ()>) -> Result<rate::Rate, ()> {
    let result = rate_provider::get();
    if let Some(rate) = result {
        let trend = get_trend(&rate, last_rate_option);

        println!("------------------------------------------------");
        print!("{}[2J", 27 as char); // Clear the screen

        println!(
            "{} {} Price USD: {} /  Price EUR: {}",
            trend,
            util::str_pad(&rate.symbol, 5, ' '),
            util::str_pad(&rate.price_usd.to_string(), 10, ' '),
            util::str_pad(&rate.price_eur.to_string(), 10, ' ')
        );

        Ok(rate)
    } else {
        Err(())
    }
}

fn get_trend(current_rate: &rate::Rate, last_rate_option: Result<rate::Rate, ()>) -> String {
    match last_rate_option {
        Ok(last_rate) => {
            if current_rate.price_usd < last_rate.price_usd {
                "v".to_string()
            } else if current_rate.price_usd > last_rate.price_usd {
                "^".to_string()
            } else {
                "o".to_string()
            }
        }
        Err(_) => "x".to_string(),
    }
}

fn main() {
    let chart = chart::Chart::new(100, 30);
    let mut last_rate_option: Result<rate::Rate, ()> = Err(());
    loop {
        chart.draw_points_with_symbol(
            vec![
                &chart::Point::new(0, 0),
                &chart::Point::new(2, 2),
                &chart::Point::new(1, 1),
                &chart::Point::new(10, 20),
                &chart::Point::new(12, 20),
                &chart::Point::new(14, 20),
                &chart::Point::new(11, 20),
                &chart::Point::new(99, 20),
                &chart::Point::new(100, 20),
                &chart::Point::new(101, 20)
            ],
            "😊"
        );

        break;

        last_rate_option = get_and_print_rates(last_rate_option);
        if last_rate_option.is_err() { break; }

        let interval = time::Duration::from_secs(1);
        thread::sleep(interval);
    }
}
