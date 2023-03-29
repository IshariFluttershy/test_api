use binance::futures::model::KlineSummary;

static mut KLINE_TIME: i64 = 0;

#[derive(Debug)]
pub struct WPattern {
    pub start_index: usize,
    pub end_index: usize,
    pub end_time: i64,
    pub lower_price: f64,
    pub neckline_price: f64
}

#[derive(Debug)]
pub struct MPattern {
    pub start_index: usize,
    pub end_index: usize,
    pub end_time: i64,
    pub higher_price: f64,
    pub neckline_price: f64
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

pub fn find_w_pattern(vec: &Vec<MathKLine>) -> Option<WPattern>{
    let start_index: usize;
    let second_v_index: usize;
    let end_index: usize;
    let neckline_index: usize;
    let lower_price: f64;
    let neckline_price: f64;
    let end_time: i64;


    // Not enough KLines or upward trend
    if vec.len() < 5 || vec[0].close > vec[0].open{
        return None;
    }

    // Get start of new upward trend
    start_index = if let Option::Some(result) = vec.iter().position(|elem| elem.close > elem.open) {
        lower_price = vec[result].low;
        result
    } else {
        return None;
    };

    // Get neckline KLine
    neckline_index = if let Option::Some(result) = &vec[start_index..].iter().position(|elem| elem.open > elem.close) {
        neckline_price = vec[*result-1].high;
        *result + start_index
    } else {
        return None;
    };

    // Find the continuation on upward trend + check if lower price breaks
    second_v_index = if let Some(result) = &vec[neckline_index..].iter().position(|elem| elem.close > elem.open) {
        if vec[*result].low < lower_price {
            return None;
        }
        *result + neckline_index
    } else {
        return None;
    };

    // Find the KLine that breaks the neckline price
    end_index = if let Some(result) = &vec[second_v_index..].iter().position(|elem| elem.high > neckline_price) {
        end_time = vec[*result].close_time;
        *result + second_v_index
    } else {
        return None;
    };

    Some(WPattern { start_index, end_index, end_time, lower_price, neckline_price })
}

pub fn find_m_pattern(vec: &Vec<MathKLine>) -> Option<MPattern>{
    let start_index: usize;
    let second_v_index: usize;
    let end_index: usize;
    let neckline_index: usize;
    let higher_price: f64;
    let neckline_price: f64;
    let end_time: i64;

    // Not enough KLines or upward trend
    if vec.len() < 5 || vec[0].close < vec[0].open{
        return None;
    }

    // Get start of new upward trend
    start_index = if let Option::Some(result) = vec.iter().position(|elem| elem.close < elem.open) {
        higher_price = vec[result].high;
        result
    } else {
        return None;
    };

    // Get neckline KLine
    neckline_index = if let Option::Some(result) = &vec[start_index..].iter().position(|elem| elem.open < elem.close) {
        neckline_price = vec[*result-1].low;
        *result + start_index
    } else {
        return None;
    };

    // Find the continuation on upward trend + check if lower price breaks
    second_v_index = if let Some(result) = &vec[neckline_index..].iter().position(|elem| elem.close < elem.open) {
        if vec[*result].high > higher_price {
            return None;
        }
        *result + neckline_index
    } else {
        return None;
    };

    // Find the KLine that breaks the neckline price
    end_index = if let Some(result) = &vec[second_v_index..].iter().position(|elem| elem.low < neckline_price) {
        end_time = vec[*result].close_time;
        *result + second_v_index
    } else {
        return None;
    };

    Some(MPattern { start_index, end_index, end_time, higher_price, neckline_price })
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