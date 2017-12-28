use rate;
use rate::RateSeries;
use term_style::style as color;

#[allow(unused)]
pub fn get_trend_from_time_series(current_rate: &rate::Rate, time_series: &RateSeries) -> i8 {
    get_trend(current_rate, &time_series.last().cloned())
}

#[allow(unused)]
pub fn get_trend(current_rate: &rate::Rate, last_rate: &Option<rate::Rate>) -> i8 {
    match last_rate {
        &Some(ref last_rate) => {
            if current_rate.price_usd < last_rate.price_usd {
                -1
            } else if current_rate.price_usd > last_rate.price_usd {
                1
            } else {
                0
            }
        }
        &None => 2,
    }
}

#[allow(unused)]
pub fn get_trend_sign_from_time_series(current_rate: &rate::Rate, time_series: &RateSeries, colors: bool) -> String {
    get_trend_sign(current_rate, &time_series.last().cloned(), colors)
}

#[allow(unused)]
pub fn get_trend_sign(current_rate: &rate::Rate, last_rate: &Option<rate::Rate>, colors: bool) -> String {
    let trend = get_trend(current_rate, &last_rate);

    if colors {
        match trend {
            -1 => color::red("▼"),
            1 => color::green("▲"),
            0 => " ".to_string(),
            _ => " ".to_string(),
        }
    } else {
        match trend {
            -1 => "▼".to_string(),
            1 => "▲".to_string(),
            0 => " ".to_string(),
            _ => "x".to_string(),
        }
    }
}
