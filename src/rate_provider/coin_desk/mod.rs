mod coin_desk_rate;

use serde_json;
use rate;
use self::coin_desk_rate::Rate as InternalRate;

pub struct CoinDesk {}

impl CoinDesk {
    fn convert_to_internal_rate(response: &str) -> Option<InternalRate> {
        let deserialized_result: serde_json::Result<InternalRate> = serde_json::from_str(&response);

        match deserialized_result {
            Ok(deserialized) => Some(deserialized),
            Err(e) => {
                print!("{:?}", e);
                None
            }
        }
    }
}

impl super::RateProvider for CoinDesk {
    fn get() -> Option<Vec<rate::Rate>> {
        let response = Self::download("https://api.coindesk.com/v1/bpi/currentprice/EUR.json");
        Self::convert(&response)
    }
    fn convert(response: &str) -> Option<Vec<rate::Rate>> {
        let internal_rate = Self::convert_to_internal_rate(response);
        if let Some(internal_rate) = internal_rate {
            return Some(vec![rate::Rate::new(
                internal_rate.bpi.usd.rate,
                internal_rate.bpi.eur.rate,
            )]);
        }

        None
    }
}