mod patterns;
mod backtest;

use binance::model::KlineSummaries;
use binance::model::KlineSummary;
use binance::futures::market::FuturesMarket;
use binance::api::*;
use binance::config::*;
use binance::futures::*;
use binance::account::*;
use binance::futures::account::*;
use crate::patterns::*;
use crate::backtest::*;


fn main() {
    let futures_api_key = Some("6e2439bdb37395afb6d6a6a7d33c93811c0dc2f4900e0638ff375ba66d63fae8".into());
    let futures_secret_key = Some("56e8e3153a0503f45ab6e88614e3a313093b28487a748e87f320c2a9c10a43f1".into());

    let config = Config::default().set_futures_rest_api_endpoint("https://testnet.binancefuture.com");
    let account: FuturesAccount = Binance::new_with_config(futures_api_key, futures_secret_key, &config);
    let market: FuturesMarket = Binance::new_with_config(None, None, &config);


    match account.account_balance() {
        Ok(answer) => println!("{:#?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    let price: Option<f64> = match market.get_price("BTCUSDT") {
        Ok(answer) => {
            println!("{:#?}", answer);
            Some(answer.price)
        }
        Err(e) => {
            println!("Error: {:?}", e);
            None
        }
    };

    let vec_w = unsafe {
        vec![
            create_test_kline(13., 12.), // 0
            create_test_kline(12., 11.), // 1
            create_test_kline(11., 10.), // 2
            create_test_kline(10., 11.), // 3
            create_test_kline(11., 12.), // 4
            create_test_kline(12., 11.), // 5
            create_test_kline(11., 12.), // 6
            create_test_kline(12., 13.), // 7
            create_test_kline(13., 14.), // 8
            create_test_kline(14., 15.)  // 9
        ]
    };

    let vec_m = unsafe {
        vec![
            create_test_kline(10., 11.), // 0
            create_test_kline(11., 12.), // 1
            create_test_kline(12., 13.), // 2
            create_test_kline(13., 12.), // 3
            create_test_kline(12., 11.), // 4
            create_test_kline(11., 12.), // 5
            create_test_kline(12., 11.), // 6
            create_test_kline(11., 10.), // 7
            create_test_kline(10., 9.), // 8
            create_test_kline(9., 8.)  // 9
        ]
    };
    let result = find_w_pattern(&vec_w);
    println!("{:#?}", result);
    let result = find_w_pattern(&vec_m);
    println!("{:#?}", result);
    let result = find_m_pattern(&vec_w);
    println!("{:#?}", result);
    let result = find_m_pattern(&vec_m);
    println!("{:#?}", result);

    let klines: Option<KlineSummaries> = match market.get_klines("BTCUSDT", "1m", 1000, None, None) {
        Ok(answer) => {
            println!("{:#?}", answer);
            Some(answer)
        }
        Err(e) => {
            println!("Error: {:?}", e);
            None
        }
    };


    let mut backtester = Backtester::new(klines.unwrap());
    backtester.start();
    println!("winratio == {}%", backtester.get_closed_ratio(TradeResult::Win));
    println!("lossratio == {}%", backtester.get_closed_ratio(TradeResult::Lost));
    println!("unknownratio == {}%", backtester.get_closed_ratio(TradeResult::Unknown));


    /*match account.market_buy("BTCUSDT", 0.1) {
        Ok(answer) => {
            println!("{:#?}", answer);
            match account.stop_market_close_sell("BTCUSDT", price.unwrap()-500.0) {
                Ok(answer) => println!("{:#?}", answer),
                Err(e) => println!("Error: {:?}", e),
            }
            match account.custom_order(tp_market_close("BTCUSDT", price.unwrap()+500.0, OrderSide::Sell)) {
                Ok(answer) => println!("{:#?}", answer),
                Err(e) => println!("Error: {:?}", e),
            }
        }
        Err(e) => println!("Error: {:?}", e),
    }*/
}

fn tp_market_close(symbol: &str, stop_price: f64, side: OrderSide) -> CustomOrderRequest {
    CustomOrderRequest {
        symbol: symbol.into(),
        side: side,
        position_side: None,
        order_type: account::OrderType::TakeProfitMarket,
        time_in_force: None,
        qty: None,
        reduce_only: None,
        price: None,
        stop_price: Some(stop_price.into()),
        close_position: Some(true),
        activation_price: None,
        callback_rate: None,
        working_type: None,
        price_protect: None,
    }
}