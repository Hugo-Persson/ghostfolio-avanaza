use std::error::Error;

use lazy_static::lazy_static;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvanzaFundInfo {
    pub isin: String,
    pub name: String,
    pub description: String,
    pub nav: f64,
    pub nav_date: String,
    pub currency: String,
    // pub rating: i64,
    // pub product_fee: f64,
    // pub management_fee: f64,
    // pub risk: i64,
    // pub risk_text: String,
    // pub development_one_day: f64,
    // pub development_one_month: f64,
    // pub development_three_months: f64,
    // pub development_six_months: f64,
    // pub development_one_year: f64,
    // pub development_this_year: f64,
    // pub development_three_years: f64,
    // pub development_five_years: f64,
    pub country_chart_data: Vec<CountryChartDaum>,
    // pub holding_chart_data: Vec<HoldingChartDaum>,
    pub sector_chart_data: Vec<SectorChartDaum>,
    // pub low_carbon: bool,
    // pub index_fund: bool,
    // pub sharpe_ratio: f64,
    // pub standard_deviation: f64,
    // pub capital: f64,
    // pub start_date: String,
    // pub fund_managers: Vec<FundManager>,
    // pub admin_company: AdminCompany,
    // pub pricing_frequency: String,
    // pub prospectus_link: String,
    // pub aum_covered_carbon: f64,
    // pub fossil_fuel_involvement: f64,
    // pub carbon_risk_score: f64,
    // pub categories: Vec<String>,
    // pub fund_type_name: String,
    // pub fund_type: String,
    // pub primary_benchmark: String,
    // pub hedge_fund: bool,
    // pub ucits_fund: bool,
    // pub recommended_holding_period: String,
    // pub portfolio_date: String,
    // pub ppm_code: String,
    // pub superloan_orderbook: bool,
    // pub esg_score: f64,
    // pub environmental_score: f64,
    // pub social_score: f64,
    // pub governance_score: f64,
    // pub controversy_score: Value,
    // pub carbon_solutions_involvement: f64,
    // pub product_involvements: Vec<ProductInvolvement>,
    // pub sustainability_rating: i64,
    // pub sustainability_rating_category_name: String,
    // pub svanen: bool,
    // pub fund_rating_views: Vec<FundRatingView>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CountryChartDaum {
    pub name: String,
    pub y: f64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub country_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HoldingChartDaum {
    pub name: String,
    pub y: f64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub currency: String,
    pub country_code: String,
    pub isin: String,
    pub orderbook_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SectorChartDaum {
    pub name: String,
    pub y: f64,
    // #[serde(rename = "type")]
    // pub type_field: String,
    // pub currency: String,
    // pub country_code: Value,
    // pub isin: Value,
    // pub orderbook_id: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundManager {
    pub name: String,
    pub start_date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminCompany {
    pub name: String,
    pub country: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductInvolvement {
    pub product: String,
    pub product_description: String,
    pub value: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundRatingView {
    pub date: String,
    pub fund_rating_type: String,
    pub fund_rating: i64,
}

lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
}

pub async fn get_avanza_fund_info(orderbook_id: &String) -> Result<AvanzaFundInfo, Box<dyn Error>> {
    let url = format!(
        "https://www.avanza.se/_api/fund-guide/guide/{}",
        orderbook_id
    );
    let response = CLIENT.get(url).send().await?;
    // Check if the request was successful (status code 200)
    if response.status().is_success() {
        let parsed_response: AvanzaFundInfo = response.json().await?;
        Ok(parsed_response)
    } else {
        Err("Errr".into())
    }
}