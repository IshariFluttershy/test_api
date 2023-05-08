use std::sync::Arc;
use std::collections::HashMap;

use crate::backtest::*;
use crate::patterns::*;

#[derive(Copy, Clone, Debug)]
pub struct StrategyParams {
    pub tp_multiplier: f64,
    pub sl_multiplier: f64,
    pub name: StrategyName,
}

pub fn create_wpattern_trades(
    chunk: Vec<MathKLine>,
    strategy_params: StrategyParams,
    patterns_params: Arc<Vec<Arc<dyn PatternParams>>>,
) -> Vec<Trade> {
    let mut result_vec = Vec::new();
    let mut j = 0;

    while j < chunk.len() {
        if let Some(wpattern_params) = patterns_params[0].downcast_ref::<WPatternParams>() {
            if let Some(result) = find_w_pattern(&chunk[j..], *wpattern_params) {
                    j += result.end_index;
                    result_vec.push(Trade {
                        entry_price: result.neckline_price,
                        sl: result.lower_price * strategy_params.sl_multiplier,
                        tp: result.neckline_price
                            + (result.neckline_price - result.lower_price)
                                * strategy_params.tp_multiplier,
                        open_time: result.end_time,
                        opening_kline: chunk[j].clone(),
                        close_time: 0,
                        closing_kline: None,
                        status: Status::NotOpened,
                        strategy: strategy_params.name,
                    });
            } else {
                j += 1;
            }
        }
    }
    result_vec
}

pub fn create_mpattern_trades(
    chunk: Vec<MathKLine>,
    strategy_params: StrategyParams,
    patterns_params: Arc<Vec<Arc<dyn PatternParams>>>,
) -> Vec<Trade> {
    let mut result_vec = Vec::new();
    let mut j = 0;
    while j < chunk.len() {
        if let Some(mpattern_params) = patterns_params[0].downcast_ref::<MPatternParams>() {
            if let Some(result) = find_m_pattern(&chunk[j..], *mpattern_params) {
                    j += result.end_index;
                    result_vec.push(Trade {
                        entry_price: result.neckline_price,
                        sl: result.higher_price * strategy_params.sl_multiplier,
                        tp: result.neckline_price + ((result.neckline_price - result.higher_price) * strategy_params.tp_multiplier),
                        open_time: result.end_time,
                        opening_kline: chunk[j].clone(),
                        close_time: 0,
                        closing_kline: None,
                        status: Status::NotOpened,
                        strategy: strategy_params.name,
                    });
            } else {
                j += 1;
            }
        }
    }
    result_vec
}

pub fn create_bull_reversal_trades(
    chunk: Vec<MathKLine>,
    strategy_params: StrategyParams,
    patterns_params: Arc<Vec<Arc<dyn PatternParams>>>,
) -> Vec<Trade> {
    let mut result_vec = Vec::new();
    let mut j = 0;
    while j < chunk.len() {
        if let Some(reversal_pattern_params) = patterns_params[0].downcast_ref::<ReversalPatternParams>() {
            if let Some(result) = find_bull_reversal(&chunk[j..], *reversal_pattern_params) {
                    j += result.end_index;
                    result_vec.push(Trade {
                        entry_price: result.end_price,
                        sl: result.peak_price * strategy_params.sl_multiplier,
                        tp: result.end_price
                            + ((result.end_price - result.peak_price)
                                * strategy_params.tp_multiplier),
                        open_time: result.end_time,
                        opening_kline: chunk[j].clone(),
                        close_time: 0,
                        closing_kline: None,
                        status: Status::NotOpened,
                        strategy: strategy_params.name,
                    });
            } else {
                j += 1;
            }
        }
    }
    result_vec
}