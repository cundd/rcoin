use rand;
use rand::distributions::{IndependentSample, Range};
use rate;
use super::ProviderError;
use super::RateProvider;
use super::Currency;

pub struct Faker {}

impl Faker {}

impl RateProvider for Faker {
    fn get_name() -> &'static str {
        "Faker"
    }

    fn get(currency: Currency) -> Result<rate::Rate, ProviderError> {
        let mut rng = rand::thread_rng();
        Ok(match currency {
            Currency::Bitcoin => rate::Rate::new(
                currency,
                Range::new(15_000.0f32, 21_000.0f32).ind_sample(&mut rng),
                Range::new(13_000.0f32, 15_000.0f32).ind_sample(&mut rng),
            ),
            Currency::Ethereum => rate::Rate::new(
                currency,
                Range::new(1_050.0f32, 1_200.0f32).ind_sample(&mut rng),
                Range::new(810.0f32, 960.0f32).ind_sample(&mut rng),
            ),
            Currency::Litecoin => rate::Rate::new(
                currency,
                Range::new(150.0f32, 360.0f32).ind_sample(&mut rng),
                Range::new(128.0f32, 306.0f32).ind_sample(&mut rng),
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_test() {
        let result: Result<rate::Rate, ProviderError> = <Faker as RateProvider>::get(Currency::Bitcoin);
        assert!(result.is_ok())
    }
}
