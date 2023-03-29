use binance::model::KlineSummaries;
use binance::model::KlineSummary;
use crate::patterns::*;

#[derive(PartialEq, Debug)]
enum Status {
    NotOpened,
    NotTriggered,
    Running,
    Won,
    Lost,
    Unknown
}

struct Trade {
    entry_price: f64,
    sl: f64,
    tp: f64,
    status: Status,
    open_time:i64,
}

pub struct Backtester {
    klines_data: Vec<MathKLine>,
    trades: Vec<Trade>
}

impl Backtester {
    pub fn new(klines_data: KlineSummaries) -> Self {
        Backtester {
            klines_data: Self::toAllMathKLine(klines_data),
            trades: Vec::new()
        }
    }

    pub fn start(&mut self) {
        self.create_trades();
        self.resolve_trades();
    }

    fn create_trades(&mut self) {
        let mut i = 0;
        while let Some(result) = find_w_pattern(&self.klines_data) {
            i = result.end_index;
            self.trades.push(Trade{
                entry_price: result.neckline_price,
                sl: result.neckline_price + 1.,
                tp: result.neckline_price + 1.,
                open_time: result.end_time,
                status: Status::NotOpened
            });
        }
        i = 0;
        while let Some(result) = find_m_pattern(&self.klines_data) {
            i = result.end_index;
            self.trades.push(Trade{
                entry_price: result.neckline_price,
                sl: result.neckline_price + 1.,
                tp: result.neckline_price + 1.,
                open_time: result.end_time,
                status: Status::NotOpened
            });
        }
    }

    fn resolve_trades(&mut self) {
        for kline in &self.klines_data {
            self.trades.iter_mut().for_each(|trade| {
                if kline.close_time == trade.open_time && trade.status == Status::NotOpened {
                    trade.status = Status::NotTriggered;
                    if trade.entry_price <= kline.high && trade.entry_price >= kline.low && trade.status == Status::NotTriggered{
                        trade.status = Status::Running;
                    }
                }

                if kline.close_time > trade.open_time && trade.status == Status::Running {
                    if Self::hit_price(trade.sl, &kline) && Self::hit_price(trade.tp, &kline) {
                        trade.status = Status::Unknown;
                    } else if Self::hit_price(trade.sl, &kline) {
                        trade.status = Status::Lost;
                    } else if Self::hit_price(trade.tp, &kline) {
                        trade.status = Status::Won;
                    }
                }
            });
        }
        println!("All trades resolved");
    }

    fn hit_price(price: f64, kline: &MathKLine) -> bool {
        price <= kline.high && price >= kline.low
    }

    fn toMathKLine(kline: &KlineSummary) -> MathKLine{
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

    fn toAllMathKLine(klines: KlineSummaries) -> Vec<MathKLine>{
        let mut result: Vec<MathKLine> = Vec::new();
        match klines {
            KlineSummaries::AllKlineSummaries(all) => {
                for kline in all.iter() {
                    result.push(Self::toMathKLine(kline));
                }
            }
        }
        result
    }
}