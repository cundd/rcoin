extern crate curl;

mod currency;
mod coin_market_cap;
mod coin_desk;
mod blockchain_info;
mod crypto_compare;
mod provider_error;

use std::time::SystemTime;

pub use self::currency::Currency;
use serde_json;
use self::curl::easy::Easy;
use rate;
use self::provider_error::ProviderError;


trait RateProvider {
    fn get_name() -> &'static str;
    fn get(currency: Currency) -> Result<rate::Rate, ProviderError>;
    fn get_all() -> Result<Vec<rate::Rate>, ProviderError> {
        Ok(vec![])
    }

    fn download<'a>(url: &'a str) -> Result<String, ProviderError> {
        let before_download = SystemTime::now();

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

            if let Err(e) = transfer.perform() {
                return Err(ProviderError::new(e.to_string()));
            }
        }


        let difference = SystemTime::now().duration_since(before_download)
            .expect("SystemTime::duration_since failed");
        println!();
        println!("{:?}", difference);
//        println!("Download time {}", Local::now());
        Ok(output)
    }

    fn convert(response: &str) -> Result<rate::Rate, ProviderError> {
        let deserialized_result: serde_json::Result<rate::Rate> = serde_json::from_str(&response);

        match deserialized_result {
            Ok(deserialized) => Ok(deserialized),
            Err(e) => {
                Err(ProviderError::new(e.to_string()))
            }
        }
    }

    fn convert_all(response: &str) -> Result<Vec<rate::Rate>, ProviderError> {
        let deserialized_result: serde_json::Result<Vec<rate::Rate>> = serde_json::from_str(&response);

        match deserialized_result {
            Ok(deserialized) => Ok(deserialized),
            Err(e) => {
                Err(ProviderError::new(e.to_string()))
            }
        }
    }
}


//pub fn get_provider(id: &str) -> Box<&RateProvider> {
//    match id {
//        "CoinDesk"
//        | "coindesk"
//        | "coin_desk" => Box::new(coin_desk::CoinDesk::new()),
//        "CoinMarketCap"
//        | "coinmarketcap"
//        | "coin_market_cap" => Box::new(coin_market_cap::CoinMarketCap::new()),
//    }
//}

#[allow(unused)]
pub fn get<S>(provider_type: S, currency: Currency) -> Result<rate::Rate, ProviderError>
    where S: Into<String> {
    let provider_type_string = provider_type.into();
    match provider_type_string.to_lowercase().as_str() {
        "coindesk" | "coin_desk" => coin_desk::CoinDesk::get(currency),
        "coinmarketcap" | "coin_market_cap" => coin_market_cap::CoinMarketCap::get(currency),
        "blockchaininfo" | "blockchain_info" => blockchain_info::BlockchainInfo::get(currency),
        "cryptocompare" | "crypto_compare" => crypto_compare::CryptoCompare::get(currency),
        _ => Err(ProviderError::new(format!("No provider for type '{}' found", provider_type_string)))
    }
}


#[allow(unused)]
pub fn get_name<S>(provider_type: S) -> Option<&'static str>
    where S: Into<String> {
    let provider_type_string = provider_type.into();
    match provider_type_string.to_lowercase().as_str() {
        "coindesk" | "coin_desk" => Some(coin_desk::CoinDesk::get_name()),
        "coinmarketcap" | "coin_market_cap" => Some(coin_market_cap::CoinMarketCap::get_name()),
        "blockchaininfo" | "blockchain_info" => Some(blockchain_info::BlockchainInfo::get_name()),
        "cryptocompare" | "crypto_compare" => Some(crypto_compare::CryptoCompare::get_name()),
        _ => None,
    }
}


#[allow(unused)]
pub fn get_all_names() -> Vec<&'static str> {
    vec![
        coin_desk::CoinDesk::get_name(),
        coin_market_cap::CoinMarketCap::get_name(),
        blockchain_info::BlockchainInfo::get_name(),
        crypto_compare::CryptoCompare::get_name(),
    ]
}

