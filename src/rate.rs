#[derive(Serialize, Deserialize, Debug)]
pub struct Rate {
    //": "bitcoin",
    pub id: String,
    //": "Bitcoin",
    pub name: String,
    //": "BTC",
    pub symbol: String,
    //": "1.0",
    pub price_btc: String,
    //": "573.137",
    pub price_usd: f32,
    // "7278.69583695",
    pub price_eur: f32,
}

impl Rate {
    pub fn new(price_usd: f32, price_eur: f32) -> Self {
        Rate {
            id: "bitcoin".to_owned(),
            name: "Bitcoin".to_owned(),
            symbol: "BTC".to_owned(),
            price_btc: "1.0".to_owned(),
            price_usd: price_usd.clone(),
            price_eur: price_eur.clone(),
        }
    }
}
