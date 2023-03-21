use rand;
use rand::Rng;
use crate::rate;
use crate::rate::Currency;
use super::ProviderError;
use super::RateProvider;

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
                rand::thread_rng().gen_range(15_000.0f32..=21_000.0f32),
                rand::thread_rng().gen_range(13_000.0f32..=15_000.0f32),
            ),
            Currency::Ethereum => rate::Rate::new(
                currency,
                rand::thread_rng().gen_range(1_050.0f32..=1_200.0f32),
                rand::thread_rng().gen_range(810.0f32..=960.0f32),
            ),
            Currency::Litecoin => rate::Rate::new(
                currency,
                rand::thread_rng().gen_range(150.0f32..=360.0f32),
                rand::thread_rng().gen_range(128.0f32..=306.0f32),
            ),
            Currency::Ripple => rate::Rate::new(
                currency,
                rand::thread_rng().gen_range(2.21f32..=3.40f32),
                rand::thread_rng().gen_range(1.84f32..=2.82f32),
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
