#[macro_use]
extern crate serde_derive;
extern crate curl;

extern crate serde;
extern crate serde_json;

mod rate;
mod rate_provider;

use rate::Rate;
use std::{thread, time};
use std::io::stdout;
use std::io::Write;

fn get_and_print_rates() {
    let result = rate_provider::get();
    if let Some(rates) = result {
        print!("\x1bc"); // Clear the screen
        stdout().flush().unwrap();

        for rate in rates {
            println!("Name: {} ({})  Price USD: {} / Price EUR: {}", rate.name, rate.symbol, rate.price_usd, rate.price_eur);
        }
    }
}

fn main() {
    loop {
        get_and_print_rates();

        let interval = time::Duration::from_secs(1);
        thread::sleep(interval);
    }
}
