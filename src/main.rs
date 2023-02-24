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
    symbols: Vec<Symbol>,
    timezone: String
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
    margin_available: bool,
    auto_asset_exchange: Option<u32>
}
#[derive(Serialize, Deserialize, Debug)]
struct Symbol {
    symbol: String,
    pair: String,
    contract_type: ContractType,
    delivery_date: u64,
    onboard_date: u64,
    status: SymbolStatus,
    main_margin_percent: f32,
    required_margin_percent: f32,
    base_asset: String,
    quote_asset: String,
    margin_asset: String,
    price_precision: u8,
    quantity_precision: u8,
    base_asset_precision: u8,
    quote_precision: u8,
    underlying_type: UnderlyingType,
    underlying_sub_type: Vec<UnderlyingSubType>,
    settle_plan: u8,
    trigger_protect: f32,
    filters: Vec<Filter>,
    order_type: Vec<OrderType>,
    time_in_force: Vec<TimeInForce>,
    liquidation_fee: f32,
    market_take_bound: f32
}

#[derive(Serialize, Deserialize, Debug)]
enum TimeInForce {
    GTC,
    IOC,
    FOK,
    GTX
}

#[derive(Serialize, Deserialize, Debug)]
enum OrderType {
    LIMIT,
    MARKET,
    STOP,
    STOP_MARKET,
    TAKE_PROFIT,
    TAKE_PROFIT_MARKET,
    TRAILING_STOP_MARKET
}
#[derive(Serialize, Deserialize, Debug)]
struct Filter {
    filter_type: String,

}

#[derive(Serialize, Deserialize, Debug)]
enum ContractType {

}

#[derive(Serialize, Deserialize, Debug)]
enum SymbolStatus {

}

#[derive(Serialize, Deserialize, Debug)]
enum UnderlyingType {

}

#[derive(Serialize, Deserialize, Debug)]
enum UnderlyingSubType {

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
