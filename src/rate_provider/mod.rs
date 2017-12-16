extern crate curl;

use serde_json;
use self::curl::easy::Easy;

mod coin_market_cap;
mod coin_desk;
mod blockchain_info;
mod provider_error;

use rate;
use self::provider_error::ProviderError;


trait RateProvider {
    fn get() -> Result<rate::Rate, ProviderError>;
    fn get_all() -> Result<Vec<rate::Rate>, ProviderError> {
        match Self::get() {
            Ok(single) => Ok(vec![single]),
            Err(e) => Err(e)
        }
    }

    fn download<'a>(url: &'a str) -> Result<String, ProviderError> {
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
        Ok(output)
    }

    fn convert(response: &str) -> Result<rate::Rate, ProviderError>;

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
pub fn get(provider_type: &str) -> Result<rate::Rate, ProviderError> {
    match provider_type {
        "CoinDesk"
        | "coindesk"
        | "coin_desk" => coin_desk::CoinDesk::get(),
        "CoinMarketCap"
        | "coinmarketcap"
        | "coin_market_cap" => coin_market_cap::CoinMarketCap::get(),
        "BlockchainInfo"
        | "blockchaininfo"
        | "blockchain_info" => blockchain_info::BlockchainInfo::get(),
        &_ => Err(ProviderError::new(format!("No provider for type '{}' found", provider_type)))
    }
}
