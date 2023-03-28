use binance::futures::model::KlineSummary;

static mut KLINE_TIME: i64 = 0;

#[derive(Debug)]
pub struct WPattern {
    start_index: usize,
    end_index: usize,
    lower_price: f64,
    neckline_price: f64
}

pub struct MathKLine {
    pub open_time: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: String,
    pub close_time: i64,
    pub quote_asset_volume: String,
    pub number_of_trades: i64,
    pub taker_buy_base_asset_volume: String,
    pub taker_buy_quote_asset_volume: String,
}

pub fn find_w_pattern() -> Option<WPattern>{
    let vec = unsafe {
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

    let start_index: usize;
    let second_v_index: usize;
    let end_index: usize;
    let neckline_index: usize;
    let lower_price: f64;
    let neckline_price: f64;

    // Not enough KLines or upward trend
    if vec.len() < 5 || vec[0].close > vec[0].open{
        println!("1");
        return None;
    }

    // Get start of new upward trend
    start_index = if let Option::Some(result) = vec.iter().position(|elem| {
            println!("close = {} -- open = {}", elem.close, elem.open);
            elem.close > elem.open
        }
    ) {
        lower_price = vec[result].low;
        result
    } else {
        println!("2");
        return None;
    };

    // Get neckline KLine
    neckline_index = if let Option::Some(result) = &vec[start_index..].iter().position(|elem| elem.open > elem.close) {
        neckline_price = vec[*result-1].high;
        *result + start_index
    } else {
        println!("3");
        return None;
    };

    // Find the continuation on upward trend + check if lower price breaks
    second_v_index = if let Some(result) = &vec[neckline_index..].iter().position(|elem| elem.close > elem.open) {
        if vec[*result].low < lower_price {
            println!("4");
            return None;
        }
        *result + neckline_index
    } else {
        println!("5");
        return None;
    };

    // Find the KLine that breaks the neckline price
    end_index = if let Some(result) = &vec[second_v_index..].iter().position(|elem| elem.high > neckline_price) {
        *result + second_v_index
    } else {
        println!("6");
        return None;
    };

    Some(WPattern { start_index, end_index, lower_price, neckline_price })
}

pub unsafe fn create_test_kline(open: f64, close: f64) -> MathKLine {
    KLINE_TIME += 1;
    MathKLine{
        open_time: KLINE_TIME,
        open: open,
        high: if open > close {open + 0.5} else {close + 0.5},
        low: if open < close {open - 0.5} else {close - 0.5},
        close: close,
        volume: "".to_string(),
        close_time: KLINE_TIME+1,
        quote_asset_volume: "".to_string(),
        number_of_trades: 0,
        taker_buy_base_asset_volume: "".to_string(),
        taker_buy_quote_asset_volume: "".to_string()
    }
}