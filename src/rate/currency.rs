#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Currency {
    Bitcoin,
    Ethereum,
    Litecoin,
}

impl Currency {
    pub fn new<S>(input: S) -> Option<Self> where S: Into<String> {
        match input.into().to_lowercase().as_str() {
            "bitcoin" | "btc" => Some(Currency::Bitcoin),
            "ethereum" | "eth" => Some(Currency::Ethereum),
            "litecoin" | "ltc" => Some(Currency::Litecoin),
            _ => None
        }
    }

    pub fn name(&self) -> &str {
        match self {
            &Currency::Bitcoin => "Bitcoin",
            &Currency::Ethereum => "Ethereum",
            &Currency::Litecoin => "Litecoin",
        }
    }

    pub fn symbol(&self) -> &str {
        match self {
            &Currency::Bitcoin => "BTC",
            &Currency::Ethereum => "ETH",
            &Currency::Litecoin => "LTC",
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_test() {
        assert_eq!(Currency::Bitcoin, Currency::new("BITCOIN").unwrap());
        assert_eq!(Currency::Bitcoin, Currency::new("BitCoin").unwrap());
        assert_eq!(Currency::Bitcoin, Currency::new("Bitcoin").unwrap());
        assert_eq!(Currency::Bitcoin, Currency::new("bitcoin").unwrap());
        assert_eq!(Currency::Bitcoin, Currency::new("BTC").unwrap());
        assert_eq!(Currency::Bitcoin, Currency::new("btc").unwrap());

        assert_eq!(Currency::Ethereum, Currency::new("ETHEREUM").unwrap());
        assert_eq!(Currency::Ethereum, Currency::new("Ethereum").unwrap());
        assert_eq!(Currency::Ethereum, Currency::new("ethereum").unwrap());
        assert_eq!(Currency::Ethereum, Currency::new("ETH").unwrap());
        assert_eq!(Currency::Ethereum, Currency::new("eth").unwrap());

        assert_eq!(Currency::Litecoin, Currency::new("LITECOIN").unwrap());
        assert_eq!(Currency::Litecoin, Currency::new("Litecoin").unwrap());
        assert_eq!(Currency::Litecoin, Currency::new("litecoin").unwrap());
        assert_eq!(Currency::Litecoin, Currency::new("LTC").unwrap());
        assert_eq!(Currency::Litecoin, Currency::new("ltc").unwrap());
    }

    #[test]
    fn name_test() {
        assert_eq!("Bitcoin", Currency::Bitcoin.name());
        assert_eq!("Ethereum", Currency::Ethereum.name());
        assert_eq!("Litecoin", Currency::Litecoin.name());
    }

    #[test]
    fn symbol_test() {
        assert_eq!("BTC", Currency::Bitcoin.symbol());
        assert_eq!("ETH", Currency::Ethereum.symbol());
        assert_eq!("LTC", Currency::Litecoin.symbol());
    }
}