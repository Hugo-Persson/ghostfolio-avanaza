use std::error::Error;

use lazy_static::lazy_static;


use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvanzaStockInfo {
    pub orderbook_id: String,
    pub name: String,
    pub isin: String,
    pub instrument_id: String,
    pub quote: Quote,
    pub listing: Listing,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Listing {
    pub ticker_symbol: String,
    pub currency: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    // pub buy: f64,
    // pub sell: f64,
    pub last: f64,
    // pub highest: f64,
    // pub lowest: f64,
    // pub change: f64,
    // pub change_percent: f64,
    // pub spread: f64,
    // pub time_of_last: i64,
    // pub total_value_traded: f64,
    // pub total_volume_traded: i64,
    // pub updated: i64,
}
lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
}
pub async fn avanza_get_stock_info(orderbook_id: &str) -> Result<AvanzaStockInfo, Box<dyn Error>> {
    let url = format!(
        "https://www.avanza.se/_api/market-guide/stock/{}",
        orderbook_id
    );
    println!("Url: {}", url);
    let response = CLIENT.get(url).send().await?;
    // Check if the request was successful (status code 200)
    if response.status().is_success() {
        let parsed_response: AvanzaStockInfo = response.json().await?;
        Ok(parsed_response)
    } else {
        println!("Error: {:#?}", response.text().await?);
        Err("Errr".into())
    }
}
