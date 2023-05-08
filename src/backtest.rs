use std::collections::HashMap;
use std::io::Write;
use std::sync::Arc;
use std::time::Instant;
use std::{thread, io, fmt};

use binance::model::KlineSummary;
use crate::patterns::*;
use crate::strategies::*;

pub type StrategyFunc = fn(Vec<MathKLine>, StrategyParams, Arc<Vec<Arc<dyn PatternParams>>>) -> Vec<Trade>;
pub type Strategy = (StrategyFunc, StrategyParams, Arc<Vec<Arc<dyn PatternParams>>>);

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum StrategyName {
    None,
    W,
    M,
    BullReversal
}

impl fmt::Display for StrategyName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StrategyName::None => write!(f, "None"),
            StrategyName::W => write!(f, "W"),
            StrategyName::M => write!(f, "M"),
            StrategyName::BullReversal => write!(f, "Bull Reversal"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Status {
    NotOpened,
    NotTriggered,
    Running,
    Closed(TradeResult)
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TradeResult {
    Win,
    Lost,
    Unknown
}

#[derive(Clone, Debug)]
pub struct Trade {
    pub entry_price: f64,
    pub sl: f64,
    pub tp: f64,
    pub status: Status,
    pub open_time: i64,
    pub close_time: i64,
    pub closing_kline: Option<MathKLine>,
    pub opening_kline: MathKLine,
    pub strategy: StrategyName,
}

#[derive(Debug)]
pub struct StrategyResult{
    pub name: StrategyName,
    pub strategy_params: StrategyParams,
    pub patterns_params: HashMap<String, String>,
    pub win_ratio: f32,
    pub lose_ratio: f32,
    pub unknown_ratio: f32,
    pub total_win: usize,
    pub total_lose: usize,
    pub total_closed: usize,
    pub total_unclosed: usize,
}

pub struct Backtester {
    klines_data: Vec<MathKLine>,
    trades: Vec<Trade>,
    num_workers: usize,
    strategies: Vec<Strategy>,
    results: Vec<StrategyResult>
}

impl Backtester {
    pub fn new(klines_data: Vec<KlineSummary>, num_workers: usize) -> Self {
        Backtester {
            klines_data: Self::to_all_math_kline(klines_data),
            trades: Vec::new(),
            num_workers,
            strategies: Vec::new(),
            results: Vec::new()
        }
    }

    pub fn start(&mut self) -> &mut Self{
        let mut i = 0;
        let size = self.strategies.len();
        let start = Instant::now();

        for strategy in self.strategies.clone() {
            self.create_trades_from_strategy(strategy.clone());
            self.resolve_trades();
            self.generate_results(strategy);
            self.clean_trades();

            i += 1;
            let duration = start.elapsed();
            print!("\rTrades resolved : {}% -- Elapsed time : {}s -- Estimated total time : {}s", 100*i/size, duration.as_secs(), ((duration.as_secs_f64()/i as f64)*size as f64) as u64);
            io::stdout().flush().unwrap();
        }
        self
    }

    fn create_trades(&mut self) {
        println!("Trade creation process starts. {} klines data to process", self.klines_data.len());
        let mut i = 0;
        let size = self.strategies.len();

        
        let start = Instant::now();
        
        for strategy in self.strategies.clone() {
            self.create_trades_from_strategy(strategy);
            i+=1;
            let duration = start.elapsed();
            print!("\rAvancement : {}% -- Elapsed time : {}s -- Estimated total time : {}s", 100*i/size, duration.as_secs(), ((duration.as_secs_f64()/i as f64)*size as f64) as u64);
            io::stdout().flush().unwrap();
        }
        println!("");
        println!("created {} trades", self.trades.len());
    }

    fn create_trades_from_strategy(&mut self, strategy: Strategy) {
        let chunk_size = (self.klines_data.len() + self.num_workers - 1) / self.num_workers;
        let results = (0..self.num_workers).map(|i| {
            let start = i * chunk_size;
            let num_elements = if i < self.num_workers - 1 {
                chunk_size
            } else {
                self.klines_data.len() - i * chunk_size
            };

            let strategy_params_clone = strategy.1.clone();
            let patterns_params_clone = strategy.2.clone();

            let chunk = Vec::from(&self.klines_data[start..][..num_elements]);
            let handle = thread::spawn(move || {
                strategy.0(chunk, strategy_params_clone, patterns_params_clone)
            });

            (i, start..start + num_elements, handle)
        }).collect::<Vec<_>>();

        for (i, range, handle) in results {
            let mut partial_results = handle.join().unwrap();
            self.trades.append(&mut partial_results);
        }
    }

    fn resolve_trades(&mut self) {
        let mut i = 0;
        let size = self.klines_data.len();
        let start = Instant::now();
        //println!("Trade resolution process starts. {} trades and {} klines data to process", self.trades.len(), self.klines_data.len());
        for kline in &self.klines_data {
            self.trades.iter_mut().for_each(|trade| {
                if kline.close_time == trade.open_time && trade.status == Status::NotOpened {
                    //trade.status = Status::NotTriggered;
                    trade.status = Status::Running;
                    if trade.entry_price <= kline.high && trade.entry_price >= kline.low && trade.status == Status::NotTriggered{
                        trade.status = Status::Running;
                    }
                }

                if kline.close_time > trade.open_time && trade.status == Status::Running {
                    if Self::hit_price(trade.sl, &kline) && Self::hit_price(trade.tp, &kline) {
                        trade.status = Status::Closed(TradeResult::Unknown);
                    } else if Self::hit_price(trade.tp, &kline) {
                        trade.status = Status::Closed(TradeResult::Win);
                    } else if Self::hit_price(trade.sl, &kline) {
                        trade.status = Status::Closed(TradeResult::Lost);
                    }
                }
            });
            i +=1;
            //let duration = start.elapsed();
            //print!("\rTrades resolved : {}% -- Elapsed time : {}s -- Estimated total time : {}s", 100*i/size, duration.as_secs(), ((duration.as_secs_f64()/i as f64)*size as f64) as u64);
            //io::stdout().flush().unwrap();
        }
        //println!("All trades resolved");
    }

    fn generate_results(&mut self, strategy: Strategy) {
        let name;
        name = strategy.1.name;
        let mut patterns_params = HashMap::new();

        for params in strategy.2.as_ref() {
            patterns_params.extend(params.get_params());
        }
        
        let total_win = self.trades.iter().filter(|&trade| trade.status == Status::Closed(TradeResult::Win)).count();
        let total_lose = self.trades.iter().filter(|&trade| trade.status == Status::Closed(TradeResult::Lost)).count();
        let total_unknown = self.trades.iter().filter(|&trade| trade.status == Status::Closed(TradeResult::Unknown)).count();
        let total_closed = self.trades.iter().filter(|&trade| matches!(trade.status, Status::Closed{..})).count();
        let total_unclosed = self.trades.len() - total_closed;


        let win_ratio = total_win as f32*100./total_closed as f32;
        let lose_ratio = total_lose as f32*100./total_closed as f32;
        let unknown_ratio = total_unknown as f32*100./total_closed as f32;

        self.results.push(StrategyResult { 
            name: name,
            strategy_params: strategy.1,
            patterns_params: patterns_params,
            win_ratio: win_ratio,
            lose_ratio: lose_ratio,
            unknown_ratio: unknown_ratio,
            total_win: total_win,
            total_lose: total_lose,
            total_closed: total_closed as usize,
            total_unclosed: total_unclosed,
         });
    }

    fn clean_trades(&mut self) {
        self.trades.clear();
    }

    pub fn add_strategy(&mut self, strategy: Strategy) -> &mut Self {
        self.strategies.push(strategy);
        self
    }

    pub fn add_strategies(&mut self, strategies:&mut Vec<Strategy> ) -> &mut Self {
        self.strategies.append(strategies);
        self
    }

    pub fn get_wr_ratio(&self) -> (f32, f32, f32, usize) {
        let total_closed = self.trades.iter().filter(|&trade| matches!(trade.status, Status::Closed{..})).count() as f32;
        let win = self.trades.iter().filter(|&trade| trade.status == Status::Closed(TradeResult::Win)).count() as f32*100./total_closed;
        let loss = self.trades.iter().filter(|&trade| trade.status == Status::Closed(TradeResult::Lost)).count() as f32*100./total_closed;
        let unknown = self.trades.iter().filter(|&trade| trade.status == Status::Closed(TradeResult::Unknown)).count() as f32*100./total_closed;
        (win, loss, unknown, total_closed as usize)
    }

    pub fn get_wr_ratio_with_strategy(&self, strategy: StrategyName) -> (f32, f32, f32, usize) {
        let total_closed = self.trades.iter().filter(|&trade| matches!(trade.status, Status::Closed{..}) && trade.strategy == strategy).count() as f32;
        let win = self.trades.iter().filter(|&trade| trade.status == Status::Closed(TradeResult::Win) && trade.strategy == strategy).count() as f32*100./total_closed;
        let loss = self.trades.iter().filter(|&trade| trade.status == Status::Closed(TradeResult::Lost) && trade.strategy == strategy).count() as f32*100./total_closed;
        let unknown = self.trades.iter().filter(|&trade| trade.status == Status::Closed(TradeResult::Unknown) && trade.strategy == strategy).count() as f32*100./total_closed;
        (win, loss, unknown, total_closed as usize)
    }

    pub fn get_num_closed(&self) -> usize {
        let result = self.trades.iter().filter(|&trade| matches!(trade.status, Status::Closed{..})).count();
        result
    }

    pub fn get_num_status(&self, trade_status: Status) -> usize {
        let result = self.trades.iter().filter(|&trade| trade.status == trade_status).count();
        result
    }

    pub fn get_results(&self) -> &Vec<StrategyResult> {
        &self.results
    }

    fn hit_price(price: f64, kline: &MathKLine) -> bool {
        price <= kline.high && price >= kline.low
    }

    fn to_math_kline(kline: &KlineSummary) -> MathKLine{
        MathKLine {
            open_time: kline.open_time,
            open: kline.open.parse::<f64>().unwrap(),
            high: kline.high.parse::<f64>().unwrap(),
            low: kline.low.parse::<f64>().unwrap(),
            close: kline.close.parse::<f64>().unwrap(),
            volume: kline.volume.clone(),
            close_time: kline.close_time,
            quote_asset_volume: kline.quote_asset_volume.clone(),
            number_of_trades: kline.number_of_trades,
            taker_buy_base_asset_volume: kline.taker_buy_base_asset_volume.clone(),
            taker_buy_quote_asset_volume: kline.taker_buy_quote_asset_volume.clone()
        }
    }

    fn to_all_math_kline(klines: Vec<KlineSummary>) -> Vec<MathKLine>{
        let mut result: Vec<MathKLine> = Vec::new();
        for kline in klines.iter() {
            result.push(Self::to_math_kline(kline));
        }
        result
    }
}