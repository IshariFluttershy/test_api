use std::sync::Arc;
use std::collections::HashMap;

use downcast_rs::impl_downcast;
use downcast_rs::DowncastSync;

use crate::backtest::*;
use crate::patterns::*;

pub trait StrategyParams: DowncastSync { fn get_params(&self) -> HashMap<String, String>; }
impl_downcast!(StrategyParams);
impl StrategyParams for WStrategyParams {
    fn get_params(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert(String::from("tp_multiplier"), self.tp_multiplier.to_string());
        map.insert(String::from("sl_multiplier"), self.sl_multiplier.to_string());
        map.insert(String::from("name_multiplier"), self.name.to_string());
        map
    }
}
impl StrategyParams for MStrategyParams {
    fn get_params(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert(String::from("tp_multiplier"), self.tp_multiplier.to_string());
        map.insert(String::from("sl_multiplier"), self.sl_multiplier.to_string());
        map.insert(String::from("name_multiplier"), self.name.to_string());
        map
    }
}
impl StrategyParams for ReversalStrategyParams {
    fn get_params(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert(String::from("tp_multiplier"), self.tp_multiplier.to_string());
        map.insert(String::from("sl_multiplier"), self.sl_multiplier.to_string());
        map.insert(String::from("name_multiplier"), self.name.to_string());
        map
    }
}

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

#[derive(Copy, Clone, Debug)]
pub struct ReversalStrategyParams {
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
                        tp: result.neckline_price + ((result.neckline_price - result.higher_price) * mstrategy_params.tp_multiplier),
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
        }
    }
    result_vec
}

pub fn create_bull_reversal_trades(
    chunk: Vec<MathKLine>,
    strategy_params: Arc<dyn StrategyParams>,
    patterns_params: Arc<Vec<Arc<dyn PatternParams>>>,
) -> Vec<Trade> {
    let mut result_vec = Vec::new();
    let mut j = 0;
    while j < chunk.len() {
        if let Some(reversal_pattern_params) = patterns_params[0].downcast_ref::<ReversalPatternParams>() {
            if let Some(result) = find_bull_reversal(&chunk[j..], *reversal_pattern_params) {
                if let Some(reversal_strategy_params) = strategy_params.downcast_ref::<ReversalStrategyParams>() {
                    j += result.end_index;
                    result_vec.push(Trade {
                        entry_price: result.end_price,
                        sl: result.peak_price * reversal_strategy_params.sl_multiplier,
                        tp: result.end_price
                            + ((result.end_price - result.peak_price)
                                * reversal_strategy_params.tp_multiplier),
                        open_time: result.end_time,
                        opening_kline: chunk[j].clone(),
                        close_time: 0,
                        closing_kline: None,
                        status: Status::NotOpened,
                        strategy: reversal_strategy_params.name,
                    });
                }
            } else {
                j += 1;
            }
        }
    }
    result_vec
}