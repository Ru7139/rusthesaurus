use serde::Serialize;

pub enum TsApiName {
    Daily,
    Shibor,
}
impl TsApiName {
    pub fn into_string(&self) -> String {
        match self {
            Self::Daily => "daily".to_string(),
            Self::Shibor => "shibor".to_string(),
        }
    }
}

pub enum TsToken {
    Ru,
    Fe,
}
impl TsToken {
    pub fn into_string(&self) -> String {
        match self {
            Self::Ru => "e1c23bbb77f2cc2ae0169d5f6da2b5b0df3b685763dad71085559c5a".to_string(),
            Self::Fe => "7ec7fdbb1c5d4c384becfdc5bcc0df6932503ea1a858dbf02196dabb".to_string(),
        }
    }
}

pub enum TsParams {
    StockDailyNormal,
    Shibor,
}
impl TsParams {
    pub fn into_string(&self) -> String {
        match self {
            Self::StockDailyNormal => "ts_code, trade_date, open, high, low, close, pre_close, change, pct_chg, vol, amount".to_string(),
            Self::Shibor => "date, on, 1w, 2w, 1m, 3m, 6m, 9m, 1y".to_string()
        }
    }
}

#[derive(Serialize)]
pub struct TushareReqwestStruct {
    api_name: String,
    token: String,
    params: TushareJsonParams,
    fields: String,
}
impl TushareReqwestStruct {
    pub fn new(
        api_name: String,
        token: String,
        fields: String,
        start_date: u32,
        end_date: u32,
    ) -> TushareReqwestStruct {
        TushareReqwestStruct {
            api_name,
            token,
            params: TushareJsonParams::new(start_date, end_date),
            fields,
        }
    }
}

#[derive(Serialize)]
struct TushareJsonParams {
    start_date: u32,
    end_date: u32,
}
impl TushareJsonParams {
    fn new(start_date: u32, end_date: u32) -> TushareJsonParams {
        TushareJsonParams {
            start_date,
            end_date,
        }
    }
}

pub fn general_tushare_post(
    client: reqwest::blocking::Client,
    tushare_struct: &TushareReqwestStruct,
) -> Result<String, String> {
    let response_from_tushare = client
        .post("http://api.tushare.pro")
        .json(tushare_struct)
        .send()
        .unwrap();

    if !response_from_tushare.status().is_success() {
        println!("StatusCode: {}", response_from_tushare.status());
        return Err("Unable to receive data from tushare".into());
    } else {
        if let Ok(finance_data_that_i_want) = response_from_tushare.text() {
            println!("text len: {}", finance_data_that_i_want.len());
            return Ok(finance_data_that_i_want);
        } else {
            return Err("Failed on response.text() -> Result<String, Err>".into());
        }
    }
}

#[cfg(test)]
#[test]
#[ignore]
fn tushare_reqwest_test() {
    let client = reqwest::blocking::Client::new();
    let trs = TushareReqwestStruct::new(
        TsApiName::Daily.into_string(),
        TsToken::Ru.into_string(),
        TsParams::StockDailyNormal.into_string(),
        20251121,
        20251121,
    );

    let _ = general_tushare_post(client, &trs).unwrap();
}
