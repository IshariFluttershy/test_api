mod backtest;
mod patterns;
mod strategies;

use crate::backtest::*;
use crate::strategies::*;
use crate::patterns::*;
use binance::account::*;
use binance::api::*;
use binance::config::*;
use binance::futures::account::*;
use binance::futures::general::FuturesGeneral;
use binance::futures::*;
use binance::market::Market;
use binance::model::KlineSummaries;
use binance::model::KlineSummary;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;

const DATA_PATH: &str = "data/testdata.json";


fn main() {
    let futures_api_key =
        Some("6e2439bdb37395afb6d6a6a7d33c93811c0dc2f4900e0638ff375ba66d63fae8".into());
    let futures_secret_key =
        Some("56e8e3153a0503f45ab6e88614e3a313093b28487a748e87f320c2a9c10a43f1".into());

    let config =
        Config::default().set_futures_rest_api_endpoint("https://testnet.binancefuture.com");
    let account: FuturesAccount =
        Binance::new_with_config(futures_api_key, futures_secret_key, &config);
    //let market: FuturesMarket = Binance::new_with_config(None, None, &config);
    //let general: FuturesGeneral = Binance::new_with_config(None, None, &config);

    //let account: FuturesAccount = Binance::new_with_config(futures_api_key, futures_secret_key, &config);
    let market: Market = Binance::new(None, None);
    let general: FuturesGeneral = Binance::new(None, None);

    match account.account_balance() {
        Ok(answer) => println!("{:#?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    /*let price: Option<f64> = match market.get_price("BNBUSDT") {
        Ok(answer) => {
            println!("{:#?}", answer);
            Some(answer.price)
        }
        Err(e) => {
            println!("Error: {:?}", e);
            None
        }
    };*/

    let mut server_time = 0;
    let result = general.get_server_time();
    match result {
        Ok(answer) => {
            println!("Server Time: {}", answer.server_time);
            server_time = answer.server_time;
        }
        Err(e) => println!("Error: {}", e),
    }

    let klines;
    if let Ok(content) = fs::read_to_string(DATA_PATH) {
        println!("data file found, deserializing");
        klines = serde_json::from_str(&content).unwrap();
        println!("deserializing finished");
    } else {
        println!("NO data file found, retreiving data from Binance server");
        klines = retreive_test_data(server_time, &market);
        println!("Data retreived from the server.");
    }

    let mut pattern_params: Vec<Arc<dyn PatternParams>> = Vec::new();
    pattern_params.push(Arc::new(WPatternParams{
        klines_repetitions: 3
    }));

    let mut pattern_params_m: Vec<Arc<dyn PatternParams>> = Vec::new();
    pattern_params_m.push(Arc::new(MPatternParams{
        klines_repetitions: 3
    }));

    let mut backtester = Backtester::new(klines, 10);
    backtester
        .add_strategy((
            strategies::create_wpattern_trades,
            Arc::new(WStrategyParams {
                tp_multiplier: 2.,
                sl_multiplier: 1.,
                name: StrategyName::W
            }),
            Arc::new(pattern_params)
        ))
        .add_strategy((
            strategies::create_mpattern_trades,
            Arc::new(MStrategyParams {
                tp_multiplier: 2.,
                sl_multiplier: 1.,
                name: StrategyName::M
            }),
            Arc::new(pattern_params_m)
        ))
        .start();


    println!(
        "trades not opened == {}",
        backtester.get_num_status(Status::NotOpened)
    );
    println!(
        "trades NotTriggered == {}",
        backtester.get_num_status(Status::NotTriggered)
    );
    println!(
        "trades Running == {}",
        backtester.get_num_status(Status::Running)
    );
    println!("trades closed == {}", backtester.get_num_closed());

    println!("WR stats == {:#?}%", backtester.get_wr_ratio());
    println!(
        "WR stats for W == {:#?}%",
        backtester.get_wr_ratio_with_strategy(StrategyName::W)
    );
    println!(
        "WR stats for M == {:#?}%",
        backtester.get_wr_ratio_with_strategy(StrategyName::M)
    );

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

fn retreive_test_data(server_time: u64, market: &Market) -> Vec<KlineSummary> {
    let mut i = 10000;
    let start_i = i;
    let mut j = 0;
    let mut start_time = server_time - (i * 60 * 1000 * 1000);
    let mut end_time = server_time - ((i - 1) * 60 * 1000 * 1000);

    let mut klines = Vec::new();
    while let Ok(retreive_klines) =
        market.get_klines("BTCUSDT", "1m", 1000, start_time, end_time)
    {
        if i <= 0 {
            break;
        }
        if let KlineSummaries::AllKlineSummaries(mut retreived_vec) = retreive_klines {
            klines.append(&mut retreived_vec);
        }

        start_time = end_time + 1000 * 60;
        end_time = start_time + 60 * 1000 * 1000;

        i -= 1;
        j += 1;
        if i % 10 == 0 {
            println!("Retreived {}/{} bench of klines data", j, start_i);
        }
    }

    let serialized = serde_json::to_string_pretty(&klines).unwrap();
    let mut file = File::create(DATA_PATH).unwrap();
    file.write_all(serialized.as_bytes()).unwrap();
    klines
}

fn _tp_market_close(symbol: &str, stop_price: f64, side: OrderSide) -> CustomOrderRequest {
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
