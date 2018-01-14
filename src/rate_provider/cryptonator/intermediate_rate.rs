//{
//    "ticker": {
//        "base": "LTC",
//        "target": "USD",
//        "price": "248.16084013",
//        "volume": "286939.69094764",
//        "change": "-0.67224984"
//    },
//    "timestamp": 1515878702,
//    "success": true,
//    "error": ""
//}
#[derive(Serialize, Deserialize, Debug)]
pub struct Ticker {
    pub base: String,
    pub target: String,
    pub price: String,
    volume: String,
    change: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IntermediateRate {
    pub ticker: Ticker,
}

impl IntermediateRate {
    pub fn price(&self)->f32 {
        self.ticker.price.parse().unwrap_or(0.0)
    }
}