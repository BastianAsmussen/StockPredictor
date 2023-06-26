use linfa::prelude::{Fit, Predict};
use ndarray::Array;
use time::{Duration, OffsetDateTime};
use yahoo_finance_api as yahoo;

fn main() {
    let provider = yahoo::YahooConnector::new();
    let (start, end) = get_time(86_400 * 365 * 30); // 30 years in seconds.

    let resp = provider.get_quote_history("AAPL", start, end).unwrap();
    let quotes = resp.quotes().unwrap();

    let last_quote = quotes.last().unwrap();

    let mut data = Vec::new();

    // Generate training data for the linear regression model.
    // It will use the closing prices to predict the next closing price.
    for quote in quotes.iter() {
        data.push((quote.open, quote.adjclose));
    }

    let mut x = Vec::new();
    let mut y = Vec::new();

    for (open, close) in data.iter() {
        x.push(*close);
        y.push(*open);
    }

    let x = Array::from(x.clone())
        .into_shape((x.len(), 1))
        .expect("Failed to reshape x!");
    let y = Array::from(y);

    let dataset = linfa::Dataset::new(x, y);

    let model = linfa_linear::LinearRegression::default()
        .fit(&dataset)
        .unwrap();

    let mut last_close = last_quote.adjclose;
    for _ in 0..3 {
        let close = Array::from_elem((1, 1), last_close);
        let prediction = model.predict(&close)[0];
        let predicted_increase = calculate_increase(last_quote.adjclose, prediction);

        println!("Predicted:\t${}", prediction);
        println!("Increase:\t{}%", predicted_increase);

        last_close = prediction;
    }
}

fn get_time(seconds: u64) -> (OffsetDateTime, OffsetDateTime) {
    let now = OffsetDateTime::now_utc();
    let start = now - Duration::seconds(seconds as i64);

    (start, now)
}

fn calculate_increase(start: f64, end: f64) -> f64 {
    (end - start) / start * 100.0
}
