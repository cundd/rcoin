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
    pub price_usd: String,
    // "7278.69583695",
    pub price_eur: String,
}
