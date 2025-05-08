use lazy_static::lazy_static;
use std::sync::Arc;
use serde::Serialize;
use tokio::sync::Mutex;

// General structure for a candle
#[derive(Clone, Debug, Default, Serialize)]
pub struct Candle {
    pub open_time: i64,
    pub close_time: i64,
    pub symbol: String,
    pub timerange: String,
    pub open: f64,
    pub close: Option<f64>,
    pub high: f64,
    pub low: f64,
    pub price: Option<f64>,
    pub volume: f64,
    pub usdt_volume: f64,
}

// Store all the different timeranges we can use
lazy_static! {
    pub static ref TIMERANGES: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![
        "1m".to_string(),
        "5m".to_string(),
        "15m".to_string(),
        "30m".to_string(),
        "1h".to_string(),
        "4h".to_string(),
        "1d".to_string(),
    ]));
}
