use std::clone;
use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvanzaHistory {
    pub id: String,
    pub data_serie: Vec<DataSerie>,
    pub name: String,
    pub from_date: String,
    pub to_date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSerie {
    #[serde(rename = "x")]
    pub timestamp: i64,
    #[serde(rename = "y")]
    pub price: f64,
}
pub enum TimePeriod {
    OneMonth,
    ThreeMonths,
    OneYear,
    ThreeYears,
    FiveYears,
    Max,
}
impl TimePeriod {
    pub fn to_str(&self) -> String {
        match self {
            Self::OneMonth => "one_month".to_string(),
            Self::ThreeMonths => "three_months".to_string(),
            Self::OneYear => "one_year".to_string(),
            Self::ThreeYears => "three_years".to_string(),
            Self::FiveYears => "five_years".to_string(),
            Self::Max => "max".to_string(),
        }
    }
}

pub async fn get_history(orderbook_id: &str, time_period: &TimePeriod) -> AvanzaHistory {
    let url  = format!("https://www.avanza.se/_api/fund-guide/chart/{}/{}?raw=true", orderbook_id, time_period.to_str());
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await.unwrap();
    let parsed_response: AvanzaHistory = response.json().await.unwrap();
    parsed_response
}