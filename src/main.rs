mod backtest;
mod patterns;
mod strategies;

use crate::backtest::*;
use crate::patterns::*;
use crate::strategies::*;
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

const DATA_PATH: &str = "data/testdataPart.json";
const RESULTS_PATH: &str = "results/results.json";


struct PriceMultiplier {
    min: f64,
    max: f64,
    step: f64,
}

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

    let mut backtester = Backtester::new(klines, 1);
    //create_reversal_pattern_strategies(&mut backtester, PriceMultiplier{ min: 0.5, max: 5., step: 0.5}, PriceMultiplier{min: 1., max: 1., step: 0.01}, 3, 7, 1, 5);
    create_w_and_m_pattern_strategies(&mut backtester, PriceMultiplier{ min: 0.5, max: 4., step: 0.5}, PriceMultiplier{min: 0.9, max: 1., step: 0.05}, 1, 5, 7, 25);
    //create_w_and_m_pattern_strategies(&mut backtester, PriceMultiplier{ min: 1., max: 1., step: 1.}, PriceMultiplier{min: 1., max: 1., step: 1.}, 3, 4, 15, 15);
    backtester.start();
    println!("");

    let results = backtester.get_results();

    for result in results {
        //if result.win_ratio > 100.*rr_ratio as f32{
            println!("rr ratio: {} -- with sl mul: {} -- with tp mul: {}", result.rr_ratio, result.strategy_params.sl_multiplier, result.strategy_params.tp_multiplier);
            println!("{:#?}", result);
        //}
    }

    let results_json = serde_json::to_string_pretty(results).unwrap();
    let mut file = File::create(RESULTS_PATH).unwrap();
    file.write_all(results_json.as_bytes()).unwrap();

    /*println!(
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
        "WR stats for BullReversal == {:#?}%",
        backtester.get_wr_ratio_with_strategy(StrategyName::BullReversal)
    );*/

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

fn create_reversal_pattern_strategies(
    backtester: &mut Backtester,
    tp: PriceMultiplier,
    sl: PriceMultiplier,
    min_trend_size: usize,
    max_trend_size: usize,
    min_counter_trend_size: usize,
    max_counter_trend_size: usize,
) {
    let mut strategies: Vec<Strategy> = Vec::new();
    let mut i = tp.min;
    while i <= tp.max {
        let mut j = sl.min;
        while j <= sl.max {
            for k in min_trend_size..=max_trend_size {
                for l in min_counter_trend_size..=max_counter_trend_size {
                    let mut reversal_pattern_params: Vec<Arc<dyn PatternParams>> = Vec::new();
                    reversal_pattern_params.push(Arc::new(ReversalPatternParams {
                        trend_size: k,
                        counter_trend_size: l,
                        name: PatternName::BullReversal
                    }));
    
                    strategies.push((
                        strategies::create_bull_reversal_trades,
                        StrategyParams {
                            tp_multiplier: i,
                            sl_multiplier: j,
                            name: StrategyName::BullReversal,
                        },
                        Arc::new(reversal_pattern_params),
                    ));
                }
            }
            j += sl.step;
        }
        i += tp.step;
    }
    backtester.add_strategies(&mut strategies);
}

fn create_w_and_m_pattern_strategies(
    backtester: &mut Backtester,
    tp: PriceMultiplier,
    sl: PriceMultiplier,
    min_klines_repetitions: usize,
    max_klines_repetitions: usize,
    min_klines_range: usize,
    max_klines_range: usize,
) {
    let mut strategies: Vec<Strategy> = Vec::new();
    let mut i = tp.min;
    while i <= tp.max {
        let mut j = sl.min;
        while j <= sl.max {
            for k in min_klines_repetitions..=max_klines_repetitions {
                for l in min_klines_range..=max_klines_range {
                    let mut pattern_params_w: Vec<Arc<dyn PatternParams>> = Vec::new();
                    pattern_params_w.push(Arc::new(WPatternParams {
                        klines_repetitions: k,
                        klines_range: l,
                        name: PatternName::W
                    }));
    
                    let mut pattern_params_m: Vec<Arc<dyn PatternParams>> = Vec::new();
                    pattern_params_m.push(Arc::new(MPatternParams {
                        klines_repetitions: k,
                        klines_range: l,
                        name: PatternName::M
                    }));
    
                    strategies.push((
                        strategies::create_wpattern_trades,
                        StrategyParams {
                            tp_multiplier: i,
                            sl_multiplier: j,
                            name: StrategyName::W,
                        },
                        Arc::new(pattern_params_w),
                    ));
    
                    strategies.push((
                        strategies::create_mpattern_trades,
                        StrategyParams {
                            tp_multiplier: i,
                            sl_multiplier: j,
                            name: StrategyName::M,
                        },
                        Arc::new(pattern_params_m),
                    ));
                }
            }
            j += sl.step;
        }

        i += tp.step;
    }

    backtester.add_strategies(&mut strategies);
}

fn retreive_test_data(server_time: u64, market: &Market) -> Vec<KlineSummary> {
    let mut i = 10000;
    let start_i = i;
    let mut j = 0;
    let mut start_time = server_time - (i * 60 * 1000 * 1000);
    let mut end_time = server_time - ((i - 1) * 60 * 1000 * 1000);

    let mut klines = Vec::new();
    while let Ok(retreive_klines) = market.get_klines("BTCUSDT", "1m", 1000, start_time, end_time) {
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
