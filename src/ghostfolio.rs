use crate::config::Config;
use inquire::Select;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::string::ToString;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GhostfolioAssets {
    pub count: i64,
    pub market_data: Vec<MarketData>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketData {
    pub asset_class: String,
    pub asset_sub_class: String,
    pub comment: Option<String>,
    pub currency: String,
    pub countries_count: i64,
    pub data_source: String,
    pub name: String,
    pub symbol: String,

    pub market_data_item_count: i64,

    pub sectors_count: i64,
    pub activities_count: i64,
}

#[derive(Serialize, Deserialize)]
pub struct GhostfolioConfig {
    token: String,
    base_url: String,
}
impl GhostfolioConfig {
    fn init() -> GhostfolioConfig {
        if !inquire::Confirm::new("Ghostfolio config missing, do you want to init?")
            .prompt()
            .expect("Failed to get input")
        {
            panic!("Ghostfolio config missing");
        }
        let token = inquire::Text::new("Enter your token")
            .prompt()
            .expect("Failed to get input");
        let base_url = inquire::Text::new("Enter your base url")
            .prompt()
            .expect("Failed to get input");
        GhostfolioConfig { token, base_url }
    }
}
pub struct GhostfolioApi {
    client: reqwest::Client,
    config: GhostfolioConfig,
}

impl GhostfolioApi {
    pub fn new() -> GhostfolioApi {
        let mut config = Config::new();
        if config.ghostfolio.is_none() {
            config.ghostfolio = Some(GhostfolioConfig::init());
            config.save();
        }
        GhostfolioApi {
            client: reqwest::Client::new(),
            config: config.ghostfolio.unwrap(),
        }
    }

    pub(crate) async fn get_assets(&self) -> Vec<MarketData> {
        let url = format!("{}/admin/market-data?take=50", self.config.base_url);
        let assets: GhostfolioAssets = self
            .client
            .get(url)
            .send()
            .await
            .expect("Failed to get assets")
            .json()
            .await
            .expect("Failed to parse assets");
        assets.market_data
    }
    pub async fn select_asset(&self) -> MarketData {
        let assets = self.get_assets().await;

        let options: Vec<String> = assets.iter().map(|asset| asset.symbol.clone()).collect();
        let ans: String = Select::new("Select your symbol", options.clone())
            .prompt()
            .expect("Failed to get input");
        let index = options.iter().position(|x| *x == ans).unwrap();
        assets[index].clone()
    }

    pub async fn create_asset(&self, asset: MarketData) {
        let url = format!(
            "{}/api/v1/admin/profile-data/MANUAL/{}",
            self.config.base_url, asset.symbol
        );
        self.client
            .post(url)
            .send()
            .await
            .expect("Failed to create asset");
    }

    pub async fn update_assets(&self, asset: MarketData) {}
}
