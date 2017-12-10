extern crate curl;

use serde_json;
use self::curl::easy::Easy;

mod coin_market_cap;
mod coin_desk;

use rate;

trait RateProvider {
    fn get() -> Option<rate::Rate>;
    fn get_all() -> Option<Vec<rate::Rate>> {
        if let Some(single) = Self::get() {
            return Some(vec![single]);
        }
        None
    }

    fn download<'a>(url: &'a str) -> String {
        let mut handle = Easy::new();
        let mut data = Vec::new();
        let mut output = String::new();
        handle.url(url).unwrap();
        {
            let mut transfer = handle.transfer();
            transfer.write_function(|new_data| {
                output.push_str(&String::from_utf8_lossy(new_data));
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            }).unwrap();
            transfer.perform().unwrap();
        }
        output
    }

    fn convert(response: &str) -> Option<rate::Rate>;

    fn convert_all(response: &str) -> Option<Vec<rate::Rate>> {
        let deserialized_result: serde_json::Result<Vec<rate::Rate>> = serde_json::from_str(&response);

        match deserialized_result {
            Ok(deserialized) => Some(deserialized),
            Err(e) => {
                print!("{:?}", e);
                None
            }
        }
    }
}


pub fn get() -> Option<rate::Rate> {
    //    coin_market_cap::CoinMarketCap::get()
    coin_desk::CoinDesk::get()
}
