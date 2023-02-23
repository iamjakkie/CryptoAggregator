use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::env;

const FUTURES_BINANCE_URL:&str = "https://fapi.binance.com";

#[derive(Serialize, Deserialize, Debug)]
struct ExchangeInfo {
    exchange_filter: ExchangeFilter,
    rate_limits: Vec<RateLimit>,
    server_time: u128,
    assets: Vec<Asset>,
    
}

#[derive(Serialize, Deserialize, Debug)]
struct ExchangeFilter {

}

#[derive(Serialize, Deserialize, Debug)]
struct RateLimit {
    interval: String,
    interval_num: u32,
    limit: u32,
    rate_limit_type: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Asset {
    asset: String,
    marginAvailable: bool,
    autoAssetExchange: Option<u32>
}

async fn f_get_server_time() {
    let path = "/fapi/v1/time";
    let base = String::from(FUTURES_BINANCE_URL);

    let url = base + path;

    println!("URL: {}", url);

    let res = reqwest::get(url)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("RES: {}", res);
}

async fn f_get_perpetuals() {
    let path = "/fapi/v1/exchangeInfo";
    let base = String::from(FUTURES_BINANCE_URL);

    let url = base + path;

    let res = reqwest::get(url)
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    println!("RES: {}", res);
}
#[tokio::main]
async fn main() {
    let binance_api = env::var("BINANCE_API").unwrap();
    let binance_secret = env::var("BINANCE_SECRET").unwrap();

    f_get_server_time().await;

    f_get_perpetuals().await;

}
