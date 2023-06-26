mod data;
mod model;

use crate::data::fetcher::TimeUnit;
use crate::model::{calculate_increase, should_buy};

fn main() {
    let symbol = "AMC";
    let time = TimeUnit::Days(5);

    let pure_data = data::fetcher::fetch(symbol, &time).unwrap();
    let mapped_data = data::mapper::convert_data(&pure_data).unwrap();
    let model = model::trainer::train(&mapped_data).unwrap();

    let last_quote = pure_data.last().unwrap().1;
    let prediction = model::predictor::predict(&model, last_quote, &time).unwrap();
    let should_buy = should_buy(&model, last_quote, &time).unwrap();
    let increase = calculate_increase(pure_data.last().unwrap().1, *prediction.last().unwrap());

    println!("Symbol: {}", symbol);
    println!("Should buy? {}", should_buy);
    println!("Increase: {}%", increase);
    println!("Predicted quote: ${}", prediction.last().unwrap());
}
