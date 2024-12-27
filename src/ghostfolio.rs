use crate::config::Config;
use inquire::Select;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountResponse {
    pub accounts: Vec<Account>,
    pub transaction_count: i64,
    pub total_balance_in_base_currency: i64,
    pub total_value_in_base_currency: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub balance: i64,
    pub comment: Value,
    pub created_at: String,
    pub currency: String,
    pub id: String,
    pub is_excluded: bool,
    pub name: String,
    pub platform_id: Value,
    pub updated_at: String,
    pub user_id: String,
    #[serde(rename = "Platform")]
    pub platform: Value,
    pub transaction_count: i64,
    pub value_in_base_currency: f64,
    pub balance_in_base_currency: i64,
    pub value: f64,
}

#[derive(Serialize, Deserialize)]
pub struct GhostfolioConfig {
    token: String,
    base_url: String,
    account_mapping: HashMap<String, String>,
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
        GhostfolioConfig {
            token,
            base_url,
            account_mapping: HashMap::new(),
        }
    }
}
pub struct GhostfolioApi {
    client: reqwest::Client,
    config: GhostfolioConfig,
    full_config: Config,
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
            full_config: config,
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

    async fn get_accounts(&self) -> AccountResponse {
        let url = format!("{}/api/v1/account", self.config.base_url);
        let accounts: AccountResponse = self
            .client
            .get(url)
            .send()
            .await
            .expect("Failed to get accounts")
            .json()
            .await
            .expect("Failed to parse accounts");
        accounts
    }
    async fn select_account(&self) -> String {
        let accounts = self.get_accounts().await;
        let options: Vec<String> = accounts
            .accounts
            .iter()
            .map(|account| account.name.clone())
            .collect();
        let ans: String = Select::new("Select your account", options.clone())
            .prompt()
            .expect("Failed to get input");
        let index = options.iter().position(|x| *x == ans).unwrap();
        accounts.accounts[index].id.clone()
    }

    pub async fn get_account_mapping(&mut self, symbol: String) -> String {
        if self.config.account_mapping.contains_key(&symbol) {
            self.config.account_mapping.get(&symbol).unwrap().clone()
        } else {
            let account = inquire::Text::new("Enter account for symbol")
                .prompt()
                .expect("Failed to get input");
            self.config.account_mapping.insert(symbol, account.clone());
            self.full_config.save();
            account
        }
    }
}
