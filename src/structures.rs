use serde::Serialize;

// General structure for a candle
#[derive(Clone, Debug, Default, Serialize)]
pub struct Candle {
    pub open_time: i64,
    pub close_time: i64,
    pub symbol: String,
    pub interval: String,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub price: Option<f64>,
    pub volume: f64,
}
