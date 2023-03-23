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

#[derive(Debug)]
struct Candlestick {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
}

fn main() {
    let api_key = Some("kuSUVSVAlMuVIJAptyR3Oy982xnbmDoPunPkms6CjDQCdEqvPluMBTjihmV1zVNg".into());
    let secret_key = Some("ahArnVH2s21G6DgKQpJB9g3g7RYTuIrffAeK7qBBmPlgwDdacljt66E67cAy5SB2".into());

    let config = Config::default().set_rest_api_endpoint("https://testnet.binance.vision");
    let account: Account = Binance::new_with_config(api_key, secret_key, &config);
    let config = Config::default().set_rest_api_endpoint("https://testnet.binance.vision");
    let market: Market = Binance::new_with_config(None, None, &config);

    let result = account.get_account();
    /*match result {
        Ok(answer) => println!("{:?}", answer.balances),
        Err(e) => println!("Error: {:?}", e),
    }

    match account.get_balance("LTC") {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    /*match account.market_sell("LTCUSDT", 5) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }*/

    // Latest price for ONE symbol
    match market.get_price("BNBUSDT") {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }*/

    // last 10 5min klines (candlesticks) for a symbol:
    let mut klines = vec![];
    match market.get_klines("BNBUSDT", "5m", 10, None, None) {
        Ok(tmpKlines) => {   
            match tmpKlines {
                binance::model::KlineSummaries::AllKlineSummaries(tmpKlines) => {

                    for kline in tmpKlines.clone() {
                        println!(
                            "Open: {}, High: {}, Low: {}",
                            kline.open, kline.high, kline.low
                        )
                    }
                    //let kline: KlineSummary = klines[0].clone(); // You need to iterate over the klines
                    klines = tmpKlines.clone();
                }
            }
        },
        Err(e) => println!("Error: {}", e),
    }

	println!("klines are == {:#?}", klines.clone());

    let candles = vec![
		Candlestick { open: 13.00, high: 14.00, low: 12.50, close: 12.50 },
	    Candlestick { open: 12.50, high: 13.50, low: 11.50, close: 12.00 },
	    Candlestick { open: 12.00, high: 12.50, low: 11.00, close: 10.00 },
	    Candlestick { open: 10.00, high: 11.00, low: 9.00, close: 10.50 },
	    Candlestick { open: 10.50, high: 12.00, low: 10.00, close: 11.50 },
	    Candlestick { open: 11.50, high: 12.50, low: 11.00, close: 11.00 },
	    Candlestick { open: 11.00, high: 12.00, low: 10.50, close: 11.50 },
	    Candlestick { open: 11.50, high: 13.00, low: 11.00, close: 12.50 },
	    Candlestick { open: 12.50, high: 14.00, low: 12.00, close: 13.00 },
	    Candlestick { open: 13.00, high: 14.50, low: 12.50, close: 13.50 },
	    Candlestick { open: 13.50, high: 14.00, low: 12.50, close: 13.00 },
	    Candlestick { open: 13.00, high: 14.00, low: 12.50, close: 12.50 },
	    Candlestick { open: 12.50, high: 13.50, low: 11.50, close: 12.00 },
	    Candlestick { open: 12.00, high: 12.50, low: 11.00, close: 11.50 },
	    Candlestick { open: 11.50, high: 12.50, low: 10.50, close: 12.00 },
	];
	
	let result = is_w_pattern(&candles);
	println!("result is == {}", result);
}

// Define a function to check for a W pattern
fn is_w_pattern(candles: &[Candlestick]) -> bool {
    // Make sure we have at least 5 candles
    if candles.len() < 5 {
        return false;
    }

	let lowest = candles.into_iter().fold(None, |min, x| match min {
	    None => Some(x),
	    Some(y) => Some(if x.low < y.low { x } else { y }),
	}).unwrap();

	let highest = candles.into_iter().fold(None, |max, x| match max {
	    None => Some(x),
	    Some(y) => Some(if x.high > y.high { x } else { y }),
	}).unwrap();


	
	println!("lowest is == {:#?}", lowest);
	println!("highest is == {:#?}", highest);
	
    // Check for the first two candles forming a downward trend
    if candles[0].high > candles[1].high && candles[0].low > candles[1].low {
        let mut is_w = true;
        let mut low_point = 0.0;

        // Check for the W pattern
        for i in 2..candles.len() {
            if candles[i].high < candles[i - 1].high && candles[i].low < candles[i - 1].low {
                if is_w && candles[i].low < low_point {
                    return true;
                }
                is_w = !is_w;
                low_point = candles[i].low;
            }
        }
    }

    false
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