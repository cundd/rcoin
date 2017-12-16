//use std::collections::Vec;

#[derive(Serialize, Deserialize, Debug)]
pub struct IntermediateRate {
    //": "bitcoin",
    pub id: String,
    //": "Bitcoin",
    pub name: String,
    //": "BTC",
    pub symbol: String,
    //": "1",
    pub rank: String,
    //": "573.137",
    pub price_usd: String,
    //": "1.0",
    pub price_btc: String,
    //": "72855700.0",
    //pub 24h_volume_usd: String,
    //": "9080883500.0",
    pub market_cap_usd: String,
    //": "15844176.0",
    pub available_supply: String,
    //": "15844176.0",
    pub total_supply: String,
    //": "0.04",
    pub percent_change_1h: String,
    //": "-0.3",
    pub percent_change_24h: String,
    //": "-0.57",
    pub percent_change_7d: String,
    //": "1472762067"
    pub last_updated: String,

    // "7278.69583695",
    pub price_eur: String,
    // "3693903712.5",
    //pub 24h_volume_eur: String,
    // "121561499173"
    pub market_cap_eur: String,
}

pub type IntermediateRateCollection = Vec<IntermediateRate>;
