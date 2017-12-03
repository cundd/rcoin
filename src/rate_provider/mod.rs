extern crate curl;

use serde_json;
use self::curl::easy::Easy;
use rate::Rate;

mod coin_market_cap;


trait RateProvider {
    fn get() -> Option<Vec<Rate>>;

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


    fn convert(response: &str) -> Option<Vec<Rate>> {
        let deserialized_result: serde_json::Result<Vec<Rate>> = serde_json::from_str(&response);

        match deserialized_result {
            Ok(deserialized) => Some(deserialized),
            Err(e) => {
                print!("{:?}", e);
                None
            }
        }
    }
}


pub fn get() -> Option<Vec<Rate>> {
    coin_market_cap::CoinMarketCap::get()
}


//fn download_and_convert<'a, T>(url: &'a str) -> Option<Vec<T>>
//    where T: serde::de::Deserialize<'a> {
//    let output: String = download(url);
//    let deserialized_result: serde_json::Result<Vec<T>> = serde_json::from_str(&output);
//
//    println!("{:?}", output);
//
//    match deserialized_result {
//        Ok(deserialized) => Some(deserialized),
//        Err(e) => {
//            print!("{:?}", e);
//            None
//        }
//    }
//}
