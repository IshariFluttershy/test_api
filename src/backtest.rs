use binance::model::KlineSummaries;
use binance::model::KlineSummary;
use crate::patterns::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Strategy {
    W,
    M
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
struct Trade {
    entry_price: f64,
    sl: f64,
    tp: f64,
    status: Status,
    open_time: i64,
    close_time: i64,
    closing_kline: Option<MathKLine>,
    opening_kline: MathKLine,
    strategy: Strategy,
}

pub struct Backtester {
    klines_data: Vec<MathKLine>,
    trades: Vec<Trade>
}

impl Backtester {
    pub fn new(klines_data: Vec<KlineSummary>) -> Self {
        Backtester {
            klines_data: Self::to_all_math_kline(klines_data),
            trades: Vec::new()
        }
    }

    pub fn start(&mut self) {
        self.create_trades();
        self.resolve_trades();
    }

    fn create_trades(&mut self) {
        let mut i = 0;
        println!("Trade creation process starts. {} klines data to process", self.klines_data.len());
        while i < self.klines_data.len() {
            if let Some(result) = find_w_pattern(&self.klines_data[i..]) {
                i += result.end_index;
                //println!("result.end_index : {:#?}", i);
                self.trades.push(Trade{
                    entry_price: result.neckline_price,
                    sl: result.lower_price,
                    tp: result.neckline_price + (result.neckline_price - result.lower_price) * 2.,
                    open_time: result.end_time,
                    opening_kline: self.klines_data[i].clone(),
                    close_time: 0,
                    closing_kline: None,
                    status: Status::NotOpened,
                    strategy: Strategy::W
                });
                //println!("W pattern found : {:#?}", result);
                //println!("New trade added : {:#?}", self.trades.last());
            } else {
                i +=1;
                if i%10 == 0 {
                    println!("Created {} trades. Processed {} kline data on {}", self.trades.len(), i, self.klines_data.len());
                }
            }
        }
        /*i = 0;
        while i < self.klines_data.len() {
            if let Some(result) = find_m_pattern(&self.klines_data[i..]) {
                i += result.end_index;
                self.trades.push(Trade{
                    entry_price: result.neckline_price,
                    sl: result.higher_price,
                    tp: result.neckline_price + (result.neckline_price - result.higher_price) * 2.,
                    open_time: result.end_time,
                    opening_kline: self.klines_data[i].clone(),
                    close_time: 0,
                    closing_kline: None,
                    status: Status::NotOpened,
                    strategy: Strategy::M
                });
                //println!("New trade added : {:#?}", self.trades.last());
            } else {
                i +=1;
            }
        }*/
        println!("created {} trades", self.trades.len());
    }

    fn resolve_trades(&mut self) {
        let mut i = 0;
        println!("Trade resolution process starts. {} trades and {} klines data to process", self.trades.len(), self.klines_data.len());
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
            if i%10000 == 0 {
                println!("Resolved trades for {} kline data on {}", i, self.klines_data.len());
            }
        }
        println!("All trades resolved");
    }

    pub fn get_WR_ratio(&self) -> (f32, f32, f32, usize) {
        let total_closed = self.trades.iter().filter(|&trade| matches!(trade.status, Status::Closed{..})).count() as f32;
        let win = self.trades.iter().filter(|&trade| trade.status == Status::Closed(TradeResult::Win)).count() as f32*100./total_closed;
        let loss = self.trades.iter().filter(|&trade| trade.status == Status::Closed(TradeResult::Lost)).count() as f32*100./total_closed;
        let unknown = self.trades.iter().filter(|&trade| trade.status == Status::Closed(TradeResult::Unknown)).count() as f32*100./total_closed;
        (win, loss, unknown, total_closed as usize)
    }

    pub fn get_WR_ratio_with_strategy(&self, strategy: Strategy) -> (f32, f32, f32, usize) {
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