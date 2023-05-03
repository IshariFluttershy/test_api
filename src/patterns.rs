static mut KLINE_TIME: i64 = 0;

#[derive(Debug)]
pub struct WPattern {
    pub start_index: usize,
    pub start_time: i64,
    pub end_index: usize,
    pub end_time: i64,
    pub lower_price: f64,
    pub neckline_price: f64
}

#[derive(Debug)]
pub struct MPattern {
    pub start_index: usize,
    pub start_time: i64,
    pub end_index: usize,
    pub end_time: i64,
    pub higher_price: f64,
    pub neckline_price: f64
}

#[derive(Clone, PartialEq, Debug)]
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

#[derive(Copy, Clone, PartialEq, Debug)]
struct TestParams {
    price: Option<f64>,
}

struct TestFunction {
    function: fn (MathKLine, Option<TestParams>) -> bool,
    params: Option<TestParams>,
}


pub struct WPatternParams {
    klines_repetitions: usize,
}


pub struct MPatternParams {
    klines_repetitions: usize,
}

pub fn find_w_pattern(vec: &[MathKLine], options: WPatternParams) -> Option<WPattern>{
    let n: usize = 3;
    let start_index: usize;
    let second_v_index: usize;
    let end_index: usize;
    let neckline_index: usize;
    let lower_price: f64;
    let neckline_price: f64;
    let start_time: i64;
    let end_time: i64;

    let is_down_test = vec![TestFunction{function: is_down, params: None}];
    let is_up_test = vec![TestFunction{function: is_up, params: None}];


    // Not enough KLines or downward trend
    if vec.len() < 5 || test_multiple_klines(vec, n, &is_down_test).is_none() {
        return None;
    }

    start_time = vec[0].open_time;

    // Get start of new upward trend
    if let Some(result) = test_multiple_klines(&vec[n..], n, &is_up_test) {
        start_index = result + n;
        lower_price = vec[start_index].low;
    } else {
        return None;
    };

    // Get neckline KLine
    if let Some(result) = test_multiple_klines(&vec[start_index..], n, &is_down_test) {
        neckline_index = result + start_index;
        neckline_price = vec[neckline_index].high;
    } else {
        return None;
    };

    // Find the continuation on upward trend + check if lower price breaks
    let second_v_test = vec![
        TestFunction{function: is_up, params: None},
        TestFunction{function: is_not_breaking_price_downwards, params: Some(TestParams{price: Some(lower_price)})}
        ];
    if let Some(result) = test_multiple_klines(&vec[neckline_index..], n, &second_v_test) {
        second_v_index = result + neckline_index;
    } else {
        return None;
    };

    // Find the KLine that breaks the neckline price
    let neckline_break_test = vec![
        TestFunction{function: is_breaking_price_upwards, params: Some(TestParams{price: Some(neckline_price)})}
        ];

    if let Some(result) = test_multiple_klines(&vec[second_v_index..], n, &neckline_break_test) {
        end_index = result + second_v_index;
        end_time = vec[end_index].close_time;
    } else {
        return None;
    };

    Some(WPattern { start_index, start_time, end_index, end_time, lower_price, neckline_price })
}

pub fn find_m_pattern(vec: &[MathKLine]) -> Option<MPattern>{
    let n = 3;
    let start_index: usize;
    let second_n_index: usize;
    let end_index: usize;
    let neckline_index: usize;
    let higher_price: f64;
    let neckline_price: f64;
    let start_time: i64;
    let end_time: i64;

    let is_down_test = vec![TestFunction{function: is_down, params: None}];
    let is_up_test = vec![TestFunction{function: is_up, params: None}];


    // Not enough KLines or upward trend
    if vec.len() < 5 || test_multiple_klines(vec, n, &is_up_test).is_none() {
        return None;
    }

    start_time = vec[0].open_time;

    // Get start of new downward trend
    if let Some(result) = test_multiple_klines(&vec[n..], n, &is_down_test) {
        start_index = result + n;
        higher_price = vec[start_index].high;
    } else {
        return None;
    };

    // Get neckline KLine
    if let Some(result) = test_multiple_klines(&vec[start_index..], n, &is_up_test) {
        neckline_index = result + start_index;
        neckline_price = vec[neckline_index].low;
    } else {
        return None;
    };

    // Find the continuation on downward trend + check if higher price breaks
    let second_n_test = vec![
        TestFunction{function: is_down, params: None},
        TestFunction{function: is_not_breaking_price_upwards, params: Some(TestParams{price: Some(higher_price)})}
        ];
    if let Some(result) = test_multiple_klines(&vec[neckline_index..], n, &second_n_test) {
        second_n_index = result + neckline_index;
    } else {
        return None;
    };

    // Find the KLine that breaks the neckline price
    let neckline_break_test = vec![
        TestFunction{function: is_breaking_price_downwards, params: Some(TestParams{price: Some(neckline_price)})}
        ];

    if let Some(result) = test_multiple_klines(&vec[second_n_index..], n, &neckline_break_test) {
        end_index = result + second_n_index;
        end_time = vec[end_index].close_time;
    } else {
        return None;
    };

    Some(MPattern { start_index, start_time, end_index, end_time, higher_price, neckline_price })
}

fn test_multiple_klines(vec: &[MathKLine], repetitions: usize, tests: &[TestFunction]) -> Option<usize> {
    let mut success_count = 0;



    for i in 0..vec.len() {
        for test in tests {
            if (test.function)(vec[i].clone(), test.params) {
                success_count += 1;
            } else {
                success_count = 0;
            }
            if success_count >= repetitions {
                return Some(i-(success_count-1));
            }
        }
    }
    None
}

fn is_up(kline: MathKLine, _: Option<TestParams>) -> bool {
    kline.close > kline.open
}

fn is_down(kline: MathKLine, _: Option<TestParams>) -> bool {
    kline.close < kline.open
}

fn is_breaking_price_upwards(kline: MathKLine, params: Option<TestParams>) -> bool {
    kline.high > params.unwrap().price.unwrap()
}

fn is_breaking_price_downwards(kline: MathKLine, params: Option<TestParams>) -> bool {
    kline.low < params.unwrap().price.unwrap()
}

fn is_not_breaking_price_upwards(kline: MathKLine, params: Option<TestParams>) -> bool {
    !(kline.high > params.unwrap().price.unwrap())
}

fn is_not_breaking_price_downwards(kline: MathKLine, params: Option<TestParams>) -> bool {
    !(kline.low < params.unwrap().price.unwrap())
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