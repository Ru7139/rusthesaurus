use super::p04_deserialize_tushare_data::TsStockDayCandleWithoutFloat;
use polars::{error::PolarsResult, prelude::LazyFrame};

pub trait ConverToLazyFrame {
    fn into_lf() -> PolarsResult<LazyFrame>;
}

impl ConverToLazyFrame for Vec<TsStockDayCandleWithoutFloat> {
    fn into_lf(&self) -> PolarsResult<LazyFrame> {
        self.iter().map(|x| x)
    }
}
