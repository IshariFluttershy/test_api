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
use binance::websockets::*;
use std::sync::atomic::{AtomicBool};

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

    //let futures_api_key = Some("6e2439bdb37395afb6d6a6a7d33c93811c0dc2f4900e0638ff375ba66d63fae8".into());
    //let futures_secret_key = Some("56e8e3153a0503f45ab6e88614e3a313093b28487a748e87f320c2a9c10a43f1".into());

    let config = Config::default().set_rest_api_endpoint("https://testnet.binance.vision");
    let account: Account = Binance::new_with_config(api_key, secret_key, &config);

    match account.market_buy("BTCUSDT", 5) {
        Ok(answer) => println!("{:#?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    match account.get_account() {
        Ok(answer) => println!("{:#?}", answer.balances),
        Err(e) => println!("Error: {:?}", e),
    }

    let mut count = 0;
    let keep_running = AtomicBool::new(true); // Used to control the event loop
    let kline = format!("{}", "btcbusd@kline_1m");
    let mut web_socket = WebSockets::new(|event: WebsocketEvent| {
        match event {
            WebsocketEvent::Kline(kline_event) => {
                count += 1;
                println!("Symbol: {}, high: {}, low: {}, count: {}", kline_event.kline.symbol, kline_event.kline.high.parse::<f32>().unwrap().round(), kline_event.kline.low.parse::<f32>().unwrap().round(), count);
                if count%20 == 0 {
                    match account.market_buy("BTCBUSD", 0.001) {
                        Ok(answer) => {
                            println!("{:#?}", answer);
                            match account.limit_sell("BTCBUSD", 0.001, answer.price+5.0) {
                                Ok(answer) => println!("{:#?}", answer),
                                Err(e) => println!("Error: {:?}", e),
                            }
                            match account.limit_sell("BTCBUSD", 0.001, answer.price-5.0) {
                                Ok(answer) => println!("{:#?}", answer),
                                Err(e) => println!("Error: {:?}", e),
                            }
                        }
                        Err(e) => println!("Error: {:?}", e),
                    }
                }
            },
            _ => (),
        };
        Ok(())
    });
 
    web_socket.connect(&kline).unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running) {
        match e {
          err => {
             println!("Error: {:?}", err);
          }
        }
     }
     web_socket.disconnect().unwrap();
}





// Define a function to check for a W pattern
fn is_w_pattern(candles: &[KlineSummary]) -> bool {
    // Make sure we have at least 5 candles
    if candles.len() < 5 {
        return false;
    }

	let lowest = candles.into_iter().fold(None, |min, x| match min {
	    None => Some(x),
	    Some(y) => Some(if x.low.parse::<f32>().unwrap() < y.low.parse::<f32>().unwrap() { x } else { y }),
	}).unwrap();

	let highest = candles.into_iter().fold(None, |max, x| match max {
	    None => Some(x),
	    Some(y) => Some(if x.high.parse::<f32>().unwrap() > y.high.parse::<f32>().unwrap() { x } else { y }),
	}).unwrap();


	
	println!("lowest is == {:#?}", lowest);
	println!("highest is == {:#?}", highest);
	
    // Check for the first two candles forming a downward trend
    if candles[0].high > candles[1].high && candles[0].low > candles[1].low {
        let mut is_w = true;
        let mut low_point = 0.0;

        // Check for the W pattern
        for i in 2..candles.len() {
            if candles[i].high.parse::<f32>().unwrap() < candles[i - 1].high.parse::<f32>().unwrap() && candles[i].low.parse::<f32>().unwrap() < candles[i - 1].low.parse::<f32>().unwrap() {
                if is_w && candles[i].low.parse::<f32>().unwrap() < low_point {
                    return true;
                }
                is_w = !is_w;
                low_point = candles[i].low.parse::<f32>().unwrap();
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