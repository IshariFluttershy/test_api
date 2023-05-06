use std::sync::Arc;

use downcast_rs::impl_downcast;
use downcast_rs::DowncastSync;

use crate::backtest::*;
use crate::patterns::*;

pub trait StrategyParams: DowncastSync {}
impl_downcast!(StrategyParams);
impl StrategyParams for WStrategyParams {}
impl StrategyParams for MStrategyParams {}

#[derive(Copy, Clone, Debug)]
pub struct WStrategyParams {
    pub tp_multiplier: f64,
    pub sl_multiplier: f64,
    pub name: StrategyName,
}

#[derive(Copy, Clone, Debug)]
pub struct MStrategyParams {
    pub tp_multiplier: f64,
    pub sl_multiplier: f64,
    pub name: StrategyName,
}

pub fn create_wpattern_trades(
    chunk: Vec<MathKLine>,
    strategy_params: Arc<dyn StrategyParams>,
    patterns_params: Arc<Vec<Arc<dyn PatternParams>>>,
) -> Vec<Trade> {
    let mut result_vec = Vec::new();
    let mut j = 0;

    while j < chunk.len() {
        if let Some(wpattern_params) = patterns_params[0].downcast_ref::<WPatternParams>() {
            if let Some(result) = find_w_pattern(&chunk[j..], *wpattern_params) {
                if let Some(wstrategy_params) = strategy_params.downcast_ref::<WStrategyParams>() {
                    j += result.end_index;
                    result_vec.push(Trade {
                        entry_price: result.neckline_price,
                        sl: result.lower_price * wstrategy_params.sl_multiplier,
                        tp: result.neckline_price
                            + (result.neckline_price - result.lower_price)
                                * wstrategy_params.tp_multiplier,
                        open_time: result.end_time,
                        opening_kline: chunk[j].clone(),
                        close_time: 0,
                        closing_kline: None,
                        status: Status::NotOpened,
                        strategy: wstrategy_params.name,
                    });
                }
            } else {
                j += 1;
            }
            if result_vec.len() % 10 == 0 {
                println!(
                    "Created {} trades. Processed {} kline data on {}",
                    result_vec.len(),
                    j,
                    chunk.len()
                );
            }
        }
    }
    result_vec
}

pub fn create_mpattern_trades(
    chunk: Vec<MathKLine>,
    strategy_params: Arc<dyn StrategyParams>,
    patterns_params: Arc<Vec<Arc<dyn PatternParams>>>,
) -> Vec<Trade> {
    let mut result_vec = Vec::new();
    let mut j = 0;
    while j < chunk.len() {
        if let Some(mpattern_params) = patterns_params[0].downcast_ref::<MPatternParams>() {
            if let Some(result) = find_m_pattern(&chunk[j..], *mpattern_params) {
                if let Some(mstrategy_params) = strategy_params.downcast_ref::<MStrategyParams>() {
                    j += result.end_index;
                    result_vec.push(Trade {
                        entry_price: result.neckline_price,
                        sl: result.higher_price * mstrategy_params.sl_multiplier,
                        tp: result.neckline_price
                            + (result.neckline_price - result.higher_price)
                                * mstrategy_params.tp_multiplier,
                        open_time: result.end_time,
                        opening_kline: chunk[j].clone(),
                        close_time: 0,
                        closing_kline: None,
                        status: Status::NotOpened,
                        strategy: mstrategy_params.name,
                    });
                }
            } else {
                j += 1;
            }
            if result_vec.len() % 10 == 0 {
                println!(
                    "Created {} trades. Processed {} kline data on {}",
                    result_vec.len(),
                    j,
                    chunk.len()
                );
            }
        }
    }
    result_vec
}
