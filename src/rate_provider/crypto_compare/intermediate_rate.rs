// https://min-api.cryptocompare.com/data/price?fsym=ETH&tsyms=BTC,USD,EUR
// {"BTC":0.03652,"USD":715.63,"EUR":595.13}

#[derive(Serialize, Deserialize, Debug)]
pub struct IntermediateRate {
    #[serde(rename = "USD")]
    pub usd: f32,
    #[serde(rename = "EUR")]
    pub eur: f32,
}
