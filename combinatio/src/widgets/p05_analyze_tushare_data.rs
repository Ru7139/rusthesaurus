use super::p04_deserialize_tushare_daily_data::TsStockDayCandleWithoutFloat;
use polars::{
    df,
    error::PolarsResult,
    prelude::{IntoLazy, LazyFrame},
};

pub trait ConverToLazyFrame {
    fn into_lazyframe(&self) -> PolarsResult<LazyFrame>;
}

impl ConverToLazyFrame for Vec<TsStockDayCandleWithoutFloat> {
    fn into_lazyframe(&self) -> PolarsResult<LazyFrame> {
        let ts_code: Vec<&str> = self.iter().map(|x| x.ts_code.as_str()).collect();
        let trade_date: Vec<&str> = self.iter().map(|x| x.trade_date.as_str()).collect();
        let open: Vec<i64> = self.iter().map(|x| x.open).collect();
        let high: Vec<i64> = self.iter().map(|x| x.high).collect();
        let low: Vec<i64> = self.iter().map(|x| x.low).collect();
        let close: Vec<i64> = self.iter().map(|x| x.close).collect();
        let pre_close: Vec<i64> = self.iter().map(|x| x.pre_close).collect();
        let change: Vec<i64> = self.iter().map(|x| x.change).collect();
        let pct_chg: Vec<i64> = self.iter().map(|x| x.pct_chg).collect();
        let vol: Vec<u64> = self.iter().map(|x| x.vol).collect();
        let amount: Vec<u64> = self.iter().map(|x| x.amount).collect();

        let df = df![
            "ts_code" => ts_code,
            "trade_date" => trade_date,
            "open" => open,
            "high" => high,
            "low" => low,
            "close" => close,
            "pre_close" => pre_close,
            "change" => change,
            "pct_chg" => pct_chg,
            "vol" => vol,
            "amount" => amount,
        ]?;

        Ok(df.lazy())
    }
}

#[cfg(test)]
#[test]
#[ignore]
fn test_of_lazyframe() {
    use super::p03_tushare_single_reqwest::{
        TsApiName, TsParams, TsToken, TushareReqwestStruct, general_tushare_post,
    };
    use super::p04_deserialize_tushare_daily_data::des;

    let client = reqwest::blocking::Client::new();
    let trs = TushareReqwestStruct::new(
        TsApiName::Daily.into_string(),
        TsToken::Ru.into_string(),
        TsParams::StockDailyNormal.into_string(),
        20251124,
        20251124,
    );

    let rqs_msg = general_tushare_post(client, &trs).unwrap();
    let vec = des(rqs_msg).unwrap();

    let lazyf = vec.into_lazyframe().unwrap();

    println!("{}", lazyf.collect().unwrap());
}
