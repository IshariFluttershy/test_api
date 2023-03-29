enum Status {
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
}

pub struct Backtester {
    klines_data: KlineSummaries,
    trades: Vec<Trade>
}

impl Backtester {
    pub fn new(klines_data: KlineSummaries) -> Self {
        Backtester {
            klines_data,
            trades
        }
    }

    pub fn start() {
        
    }
}