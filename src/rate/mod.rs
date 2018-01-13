mod rate_series;
mod currency;

pub use self::rate_series::RateSeries;
pub use self::currency::Currency;
use matrix::PointTrait;
use ui::CoordinatePrecision;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rate {
    pub currency: Currency,
    pub price_btc: f32,
    pub price_usd: f32,
    pub price_eur: f32,

    x: CoordinatePrecision,
    y: CoordinatePrecision,
}

impl Rate {
    pub fn new(currency: Currency, price_usd: f32, price_eur: f32) -> Self {
        Rate {
            currency,
            price_btc: 1.0,
            price_usd: price_usd.clone(),
            price_eur: price_eur.clone(),
            x: 0,
            y: Self::price_to_coordinate_scaled(price_usd, &currency),
        }
    }

    pub fn price_to_coordinate(price: f32) -> CoordinatePrecision {
        price.round() as CoordinatePrecision
    }

    fn price_to_coordinate_scaled(price: f32, currency: &Currency) -> CoordinatePrecision {
        (price * currency.scale()).round() as CoordinatePrecision
    }
}

impl PointTrait for Rate {
    fn x(&self) -> CoordinatePrecision {
        self.x
    }

    fn y(&self) -> CoordinatePrecision {
        self.y
    }

    fn with_x(&self, new_x: CoordinatePrecision) -> Self {
        let mut clone = self.clone();
        clone.x = new_x;

        clone
    }

    fn with_y(&self, new_y: CoordinatePrecision) -> Self {
        let mut clone = self.clone();
        clone.y = new_y;

        clone
    }

    fn with_x_y(&self, new_x: CoordinatePrecision, new_y: CoordinatePrecision) -> Self {
        let mut clone = self.clone();
        clone.x = new_x;
        clone.y = new_y;

        clone
    }
}

//impl<'a> PointTrait for &'a Rate {
//    fn x(&self) -> usize {
//        self.x
//    }
//
//    fn y(&self) -> usize {
//        self.y
//    }
//
//    fn with_x(&self, new_x: usize) -> Self {
//        let mut clone = Rate {
//            id: self.id.clone(),
//            name: self.name.clone(),
//            symbol: self.symbol.clone(),
//            price_btc: self.price_btc,
//            price_usd: self.price_usd,
//            price_eur: self.price_eur,
//            x: new_x,
//            y: self.y,
//        };
//
//        &clone
//    }
//
//    fn with_y(&self, new_y: usize) -> Self {
//        let mut clone = self.clone();
//        clone.y = new_y;
//
//        clone
//    }
//
//    fn with_x_y(&self, new_x: usize, new_y: usize) -> Self {
//        let mut clone = self.clone();
//        clone.x = new_x;
//        clone.y = new_y;
//
//        clone
//    }
//}