use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::env;

const FUTURES_BINANCE_URL:&str = "https://fapi.binance.com";

#[derive(Serialize, Deserialize, Debug)]
struct ExchangeInfo {
    exchangeFilters: ExchangeFilter,
    rateLimits: Vec<RateLimit>,
    serverTime: u128,
    assets: Vec<Asset>,
    symbols: Vec<Symbol>,
    timezone: String,
    futuresType: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ExchangeFilter {

}

#[derive(Serialize, Deserialize, Debug)]
struct RateLimit {
    interval: String,
    intervalNum: u32,
    limit: u32,
    rateLimitType: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Asset {
    asset: String,
    marginAvailable: bool,
    autoAssetExchange: Option<String>
}
#[derive(Serialize, Deserialize, Debug)]
struct Symbol {
    symbol: String,
    pair: String,
    contractType: ContractType,
    deliveryDate: u64,
    onboardDate: u64,
    status: SymbolStatus,
    mainMarginPercent: f32,
    requiredMarginpercent: f32,
    baseAsset: String,
    quoteAsset: String,
    marginAsset: String,
    pricePrecision: u8,
    quantityPrecision: u8,
    baseAssetPrecision: u8,
    quotePrecision: u8,
    underlyingType: UnderlyingType,
    underlyingSubType: Vec<UnderlyingSubType>,
    settlePlan: u8,
    triggerProtect: f32,
    filters: Vec<Filter>,
    orderType: Vec<OrderType>,
    timeInForce: Vec<TimeInForce>,
    liquidationFee: f32,
    marketTakeBound: f32
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
    filterType: String,
    maxPrice: Option<String>,
    minPrice: Option<String>,
    tickSize: Option<String>,
    maxQty: Option<String>,
    minQty: Option<String>,
    stepSize: Option<String>,
    limit: Option<u32>,
    multiplierUp: Option<String>,
    multiplierDown: Option<String>,
    multiplierDecimal: Option<String>,

}

#[derive(Serialize, Deserialize, Debug)]
enum ContractType {
    PERPETUAL,
}

#[derive(Serialize, Deserialize, Debug)]
enum SymbolStatus {
    TRADING,
}

#[derive(Serialize, Deserialize, Debug)]
enum UnderlyingType {
    COIN
}

#[derive(Serialize, Deserialize, Debug)]
enum UnderlyingSubType {
    STORAGE
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
        .text()
        .await
        .unwrap();

    println!("{:?}", &res[..2000]);

    let resp_formatted:Value = serde_json::from_str(&res).unwrap();
    
    let timezone = serde_json::from_value::<String>(resp_formatted["timezone"].clone()).unwrap();
    let server_time = serde_json::from_value::<u128>(resp_formatted["serverTime"].clone()).unwrap();
    let futures_type = serde_json::from_value::<String>(resp_formatted["futuresType"].clone()).unwrap();
    /* 
        "rateLimits": [
            {
                "interval": "MINUTE",
                "intervalNum": 1,
                "limit": 2400,
                "rateLimitType": "REQUEST_WEIGHT" 
            },
            {
                "interval": "MINUTE",
                "intervalNum": 1,
                "limit": 1200,
                "rateLimitType": "ORDERS"
            }
        ],
    */
    let rate_limits = serde_json::from_value::<Vec<RateLimit>>(resp_formatted["rateLimits"].clone()).unwrap();
    let assets = serde_json::from_value::<Vec<Asset>>(resp_formatted["assets"].clone()).unwrap();
    let symbols = serde_json::from_value::<Vec<Symbol>>(resp_formatted["symbols"].clone()).unwrap();

    println!("Timezone: {}", timezone);
    println!("Server Time: {}", server_time);
    println!("Futures Type: {}", futures_type);
    println!("Rate limits: {:?}", rate_limits);
    println!("assets: {:?}", assets);
    println!("symbols: {:?}", symbols);
    // println!("resp_formatted: {}", resp_formatted["serverTime"]);

    // let resp_result = serde_json::from_value::<u128>(resp_formatted["serverTime"].clone()).unwrap();
    // println!("resp_result: {}", resp_result);


}
#[tokio::main]
async fn main() {
    let binance_api = env::var("BINANCE_API").unwrap();
    let binance_secret = env::var("BINANCE_SECRET").unwrap();

    f_get_server_time().await;

    f_get_perpetuals().await;

}
