use serde::{Serialize, Deserialize};

//use std::collections::HashMap;
// https://api.coindesk.com/v1/bpi/currentprice/CNY.json

//{
//    "bpi": {
//        "CNY": {
//            "code": "CNY",
//            "description": "Chinese Yuan",
//            "rate": "40,202.5000",
//            "rate_float": 40202.5
//        },
//        "USD": {
//            "code": "USD",
//            "description": "United States Dollar",
//            "rate": "11,740.1125",
//            "rate_float": 11740.1125
//        }
//    },
//    "disclaimer": "This data was produced from the CoinDesk Bitcoin Price Index (USD & CNY respectively).",
//    "time": {
//        "updated": "Dec 3, 2017 19:05:00 UTC",
//        "updatedISO": "2017-12-03T19:05:00+00:00",
//        "updateduk": "Dec 3, 2017 at 19:05 GMT"
//    }
//}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bpi {
    pub code: String,
    #[serde(rename = "rate_float")]
    pub rate: f32,
}

// Flexible type
//pub type BpiMap = HashMap<String, Bpi>;

// Strict type - easy for mapping
#[derive(Serialize, Deserialize, Debug)]
pub struct BpiMap {
    #[serde(rename = "USD")]
    pub usd: Bpi,
    #[serde(rename = "EUR")]
    pub eur: Bpi,
}

#[derive(Serialize, Deserialize, Debug)]
struct Time {
    updated: String,
    #[serde(rename = "updatedISO")]
    updated_iso: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rate {
    time: Time,
    pub bpi: BpiMap,
}
