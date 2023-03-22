use std::error::Error;
use std::fs;
use hmac::{Hmac, Mac};
use serde::{Deserialize};
use binance::api::*;
use binance::savings::*;
use binance::config::*;
use binance::general::*;
use binance::account::*;
use binance::market::*;
use binance::model::KlineSummary;
use binance::errors::ErrorKind as BinanceLibErrorKind;



fn main() {
    let api_key = Some("kuSUVSVAlMuVIJAptyR3Oy982xnbmDoPunPkms6CjDQCdEqvPluMBTjihmV1zVNg".into());
    let secret_key = Some("ahArnVH2s21G6DgKQpJB9g3g7RYTuIrffAeK7qBBmPlgwDdacljt66E67cAy5SB2".into());

    let config = Config::default().set_rest_api_endpoint("https://testnet.binance.vision");
    let account: Account = Binance::new_with_config(api_key, secret_key, &config);
    let config = Config::default().set_rest_api_endpoint("https://testnet.binance.vision");
    let market: Market = Binance::new_with_config(None, None, &config);

    let result = account.get_account();
    match result {
        Ok(answer) => println!("{:?}", answer.balances),
        Err(e) => println!("Error: {:?}", e),
    }

    match account.get_balance("LTC") {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    /*match account.market_sell("LTCUSDT", 505) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }*/

    // Latest price for ONE symbol
    match market.get_price("BNBUSDT") {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }
}










/* #[derive(Deserialize, Debug)]
struct ServerTime {
    serverTime: u64
} */


/* #[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //let public_api_key = fs::read_to_string("api_key.txt").expect("Should have been able to read the file");
    //let private_api_key = fs::read_to_string("private_api_key.txt").expect("Should have been able to read the file");
    let public_api_key = "kuSUVSVAlMuVIJAptyR3Oy982xnbmDoPunPkms6CjDQCdEqvPluMBTjihmV1zVNg";
    let private_api_key = "ahArnVH2s21G6DgKQpJB9g3g7RYTuIrffAeK7qBBmPlgwDdacljt66E67cAy5SB2";

    let mut mac = HmacSha256::new_from_slice(private_api_key)
    .expect("HMAC can take key of any size");
    mac.update(b"input message");

// `result` has type `CtOutput` which is a thin wrapper around array of
// bytes for providing constant time equality check
let result = mac.finalize();

    let client = reqwest::Client::new();
    //let req = client.get("https://testnet.binance.vision/api/v3/avgPrice")
    //    .query(&[("symbol", "BNBBTC")]);

    let req = client.get("https://testnet.binance.vision/api/v3/time");
    let resp = req.send()
        .await?;

    let time = resp
        .json::<ServerTime>()
        .await?;
    println!("response == {:#?}", time);

    let req = client.get("https://testnet.binance.vision/api/v3/account")
        .header("X-MBX-APIKEY", public_api_key)
        .query(&[("timestamp", time.serverTime)])
        .query(&[("signature", private_api_key)]);

    let resp = req.send()
        .await?
        .text()
        .await?;
    println!("response == {:#?}", resp);
    Ok(())
} */
//https://api.binance.com/api/v3/exchangeInfo?symbol=BNBBTC
//GET /sapi/v1/capital/config/getall
// POST /api/v3/order/test (HMAC SHA256)