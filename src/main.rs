#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
//extern crate curl;

mod rate;
mod rate_provider;
mod util;

//use rate::Rate;
use std::{thread, time};

fn get_and_print_rates() -> Result<(), ()> {
    let result = rate_provider::get();
    if let Some(rates) = result {
        println!("------------------------------------------------");
        print!("{}[2J", 27 as char); // Clear the screen

        for rate in rates {
            println!(
                "{} Price USD: {} /  Price EUR: {}",
                util::str_pad(&rate.symbol, 5, ' '),
                util::str_pad(&rate.price_usd.to_string(), 10, ' '),
                util::str_pad(&rate.price_eur.to_string(), 10, ' ')
            );
        }

        Ok(())
    } else {
        Err(())
    }
}

fn main() {
    loop {
        if get_and_print_rates().is_err() { break; }

        let interval = time::Duration::from_secs(1);
        thread::sleep(interval);
    }
}
