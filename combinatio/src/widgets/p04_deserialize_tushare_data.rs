use serde::Deserialize;

trait WithoutFloat {
    fn into_u32_without_float(&self) -> u32;
}

impl WithoutFloat for f32 {
    fn into_u32_without_float(&self) -> u32 {
        (self * 1000.0).round() as u32
    }
}

#[derive(Deserialize)]
pub struct TsStockDayK {
    ts_code: String,
    trade_date: String,
    open: f32,
    high: f32,
    low: f32,
    close: f32,
    pre_close: f32,
    change: f32,
    pct_chg: f32,
    vol: f32,
    amount: f32,
}
impl TsStockDayK {
    fn into_without_float_struct(self) -> TsStockDayCandleWithoutFloat {
        TsStockDayCandleWithoutFloat {
            ts_code: self.ts_code,
            trade_date: self.trade_date,
            open: self.open.into_u32_without_float(),
            high: self.high.into_u32_without_float(),
            low: self.low.into_u32_without_float(),
            close: self.close.into_u32_without_float(),
            pre_close: self.pre_close.into_u32_without_float(),
            change: self.change.into_u32_without_float(),
            pct_chg: self.pct_chg.into_u32_without_float(),
            vol: self.vol.into_u32_without_float(),
            amount: self.amount.into_u32_without_float(),
        }
    }
}

#[derive(Deserialize)]
pub struct TsStockInnerData {
    fields: Vec<String>,
    items: Vec<TsStockDayK>,
    has_more: bool,
    count: i32,
}

#[derive(Deserialize)]
pub struct TsPostStockData {
    request_id: String,
    code: i32,
    data: TsStockInnerData,
    msg: String,
}

#[derive(Debug)]
pub struct TsStockDayCandleWithoutFloat {
    ts_code: String,
    trade_date: String,
    open: u32,
    high: u32,
    low: u32,
    close: u32,
    pre_close: u32,
    change: u32,
    pct_chg: u32,
    vol: u32,
    amount: u32,
}

pub fn des(
    rqs_msg: String,
) -> Result<Vec<TsStockDayCandleWithoutFloat>, Box<dyn std::error::Error>> {
    let vec_raw: TsPostStockData = serde_json::from_str(&rqs_msg).unwrap();
    let vec_inner = vec_raw
        .data
        .items
        .into_iter()
        .map(|x| x.into_without_float_struct())
        .collect();

    Ok(vec_inner)
}

#[cfg(test)]
#[test]
fn test_of_des() {
    use super::p03_tushare_single_reqwest::{
        TsApiName, TsParams, TsToken, TushareReqwestStruct, general_tushare_post,
    };

    let client = reqwest::blocking::Client::new();
    let trs = TushareReqwestStruct::new(
        TsApiName::Daily.into_string(),
        TsToken::Ru.into_string(),
        TsParams::StockDailyNormal.into_string(),
        20251121,
        20251121,
    );

    let rqs_msg = general_tushare_post(client, &trs).unwrap();
    let vec = des(rqs_msg).unwrap();
    println!("{:#?}", vec[0]);
}
