use std::env;

const FUTURES_BINANCE_URL:&str = "https://fapi.binance.com";


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
#[tokio::main]
async fn main() {
    let binance_api = env::var("BINANCE_API").unwrap();
    let binance_secret = env::var("BINANCE_SECRET").unwrap();

    f_get_server_time().await;



}
