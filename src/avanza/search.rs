use std::error::Error;

use lazy_static::lazy_static;
use serde_json::{json, Value};

use serde::{Deserialize, Serialize};
use log::info;
use crate::SymbolType;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvanzaSearchResult {
    // pub total_number_of_hits: i64,
    pub result_groups: Vec<ResultGroup>,
    // pub page_search_results: PageSearchResults,
    // pub search_query: String,
    // pub url_encoded_search_query: String,
    // pub configuration_response: ConfigurationResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultGroup {
    pub instrument_type: String,
    // pub number_of_hits: i64,
    pub hits: Vec<Hit>,
    // pub instrument_display_name: String,
    // pub instrument_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hit {
    pub link: Link,
    pub currency: String,
    pub last_price: String,
    // pub today_change: String,
    // pub today_change_direction: String,
    // pub today_change_value: String,
    // pub one_quarter_ago_change: String,
    // pub one_quarter_ago_change_direction: String,
    pub highlighted_display_title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    #[serde(rename = "type")]
    pub type_field: String,
    //pub flag_code: String,
    pub orderbook_id: String,
    // pub tradeable: bool,
    // pub buyable: bool,
    // pub sellable: bool,
    pub url_display_name: String,
    pub link_display: String,
    pub short_link_display: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageSearchResults {
    pub total_number_of_hits: i64,
    pub number_of_hits: i64,
    pub hits: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigurationResponse {
    pub monthly_savings_url: String,
}

lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
}

fn prepare_avanza_search_body(isin: &String) -> serde_json::Value {
    json!({
        "query": isin,
        "screenSize": "PHONE",
        "originPath": "/start",
        "searchSessionId": "b1b431f8-2542-400b-bcc0-c594fa124493"
    })
}

pub async fn search_avanza(isin: &String) -> Result<Vec<Hit>, Box<dyn Error>> {
    let url = "https://www.avanza.se/_api/search/global-search?limit=10";
    let post_body = prepare_avanza_search_body(isin);
    info!("Post body: {:#?}", post_body);
    let response = CLIENT.post(url).json(&post_body).send().await?;

    // Check if the request was successful (status code 200)
    if response.status().is_success() {
        let mut results: Vec<Hit> = Vec::new();
        let mut parsed_response: AvanzaSearchResult = response.json().await?;
        parsed_response.result_groups.iter_mut().for_each(|group| {
            group.hits.iter_mut().for_each(|hit| {
                results.push(hit.clone());
            })
        });
        Ok(results)




    } else {
        Err("Errr".into())
    }
}