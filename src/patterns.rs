use binance::futures::model::KlineSummary;

static mut klineTime: i64 = 0;

pub fn find_w_pattern() -> Option<(u64, u64, f64, f64)>{


    let vec = unsafe {
        vec![
            create_test_kline(13., 12.),
            create_test_kline(12., 11.),
            create_test_kline(11., 10.),
            create_test_kline(10., 11.),
            create_test_kline(11., 12.),
            create_test_kline(12., 11.),
            create_test_kline(11., 12.),
            create_test_kline(12., 13.),
            create_test_kline(13., 14.),
            create_test_kline(14., 15.)
        ]
    };


    Some((2, 9, 100., 110.))
}

pub unsafe fn create_test_kline(open: f64, close: f64) -> KlineSummary {
    klineTime += 1;
    KlineSummary{
        open_time: klineTime,
        open: open.to_string(),
        high: (open+0.5).to_string(),
        low: (close-0.5).to_string(),
        close: open.to_string(),
        volume: "".to_string(),
        close_time: klineTime+1,
        quote_asset_volume: "".to_string(),
        number_of_trades: 0,
        taker_buy_base_asset_volume: "".to_string(),
        taker_buy_quote_asset_volume: "".to_string()
    }
}