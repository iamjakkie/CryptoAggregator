use serde_json::Value;
use std::env;

const FUTURES_BINANCE_URL:&str = "https://fapi.binance.com";


#[tokio::main]
async fn main() {
    let binance_api = env::var("BINANCE_API").unwrap();
    let binance_secret = env::var("BINANCE_SECRET").unwrap();

    f_get_server_time().await;

    f_get_perpetuals().await;

}
