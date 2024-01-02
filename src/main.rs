use crate::avanza::history::TimePeriod;
use crate::avanza::search::Hit;
use chrono::NaiveDate;
use clap::{Parser, Subcommand};
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use serde::Serialize;
use serde_json::{json, to_string};
use std::fmt;
use std::mem::zeroed;
use std::path::PathBuf;

mod avanza;

#[derive(Serialize, PartialEq, Debug)]
pub enum SymbolType {
    STOCK,
    #[serde(rename = "FUND")]
    MUTUALFUND,
}
impl fmt::Display for SymbolType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::STOCK => write!(f, "STOCK"),
            Self::MUTUALFUND => write!(f, "FUND"),
        }
    }
}
impl SymbolType {
    pub fn from_str(s: &str) -> Self {
        println!("s: {}", s);
        match s {
            "STOCK" => Self::STOCK,
            "FUND" => Self::MUTUALFUND,
            _ => panic!("Unknown symbol type"),
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
pub struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, PartialEq)]
enum Commands {
    /// Import market history for a symbol
    Import {
        name: String,

        /// From, format: YYYY-MM-DD. Defaults to 1 year ago
        #[arg(short, long)]
        from: Option<String>,

        /// To, format: YYYY-MM-DD. Defaults to today
        #[arg(short, long)]
        to: Option<String>,
    },

    ParseTransactions {
        #[arg(short, long)]
        file: PathBuf,
    },

    GetScraperConfiguration {
        name: String,
    },
}

fn get_date_one_year_ago() -> String {
    let today = chrono::offset::Local::now().naive_local();
    let one_year_ago = today - chrono::Duration::days(365);

    format!("{}", one_year_ago.format("%Y-%m-%d"))
}

fn get_today() -> String {
    let today = chrono::offset::Local::now().naive_local();
    format!("{}", today.format("%Y-%m-%d"))
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Import { name, from, to }) => {
            println!("Importing history for {}", name);
            import_history(
                name,
                from.unwrap_or(get_date_one_year_ago()),
                to.unwrap_or(get_today()),
            )
            .await;
        }
        Some(Commands::ParseTransactions { file }) => todo!("Finish this"),
        Some(Commands::GetScraperConfiguration { name }) => get_scraper_configuration(name).await,
        None => {
            println!("No command specified");
        }
    }
}

fn time_period_from(from: NaiveDate) -> TimePeriod {
    let today: NaiveDate = chrono::offset::Local::now().naive_local().into();
    let diff = today.signed_duration_since(from);
    let days = diff.num_days();
    if days < 30 {
        TimePeriod::OneMonth
    } else if days < 90 {
        TimePeriod::ThreeMonths
    } else if days < 365 {
        TimePeriod::OneYear
    } else if days < 365 * 3 {
        TimePeriod::ThreeYears
    } else if days < 365 * 5 {
        TimePeriod::FiveYears
    } else {
        TimePeriod::Max
    }
}

async fn import_history(name: String, from: String, to: String) {
    let to_timestamp = chrono::NaiveDate::parse_from_str(&to, "%Y-%m-%d").unwrap();
    let from_timestamp = chrono::NaiveDate::parse_from_str(&from, "%Y-%m-%d").unwrap();
    let hit = find_symbol(name).await;
    let mut csv_data: Vec<String> = vec!["date;marketPrice".to_string()];
    let mut time_period = time_period_from(to_timestamp);
    let mut oldest_recorded_timestamp = i64::MAX;

    loop {
        println!("period: {}", time_period.to_str());
        let history = avanza::history::get_history(&hit.link.orderbook_id, &time_period).await;
        let start_timestamp = history.data_serie[0].timestamp;
        for data in history.data_serie {
            if data.timestamp > oldest_recorded_timestamp {
                break;
            }
            let timestamp: chrono::NaiveDate =
                chrono::NaiveDateTime::from_timestamp_millis(data.timestamp)
                    .unwrap()
                    .into();

            if timestamp > to_timestamp {
                break;
            }
            csv_data.push(format!(
                "{};{}",
                timestamp_to_date(data.timestamp),
                data.price
            ));
        }
        oldest_recorded_timestamp = start_timestamp;
        if is_date_greater(&from, &history.from_date) {
            break;
        }
        time_period = match time_period {
            TimePeriod::OneMonth => TimePeriod::ThreeMonths,
            TimePeriod::ThreeMonths => TimePeriod::OneYear,
            TimePeriod::OneYear => TimePeriod::ThreeYears,
            TimePeriod::ThreeYears => TimePeriod::FiveYears,
            TimePeriod::FiveYears => TimePeriod::Max,
            TimePeriod::Max => break,
        };
    }

    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(csv_data.join("\n")).unwrap();
    println!("Copied to clipboard");
}

fn is_date_greater(first: &String, second: &String) -> bool {
    let first_date = chrono::NaiveDate::parse_from_str(first, "%Y-%m-%d").unwrap();
    let second_date = chrono::NaiveDate::parse_from_str(second, "%Y-%m-%d").unwrap();
    first_date > second_date
}
fn timestamp_to_date(timestamp: i64) -> String {
    let naive = chrono::NaiveDateTime::from_timestamp_millis(timestamp).unwrap();
    naive.format("%Y-%m-%d").to_string()
}

async fn find_symbol(name: String) -> Hit {
    let hits = avanza::search::search_avanza(&name).await.unwrap();
    let res = hits.iter().map(format_hit).collect::<Vec<String>>();
    let mut i = 1;
    println!("Select symbol:");
    for r in res {
        println!("{}. {}", i, r);
        i += 1;
    }
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let index = input.trim().parse::<usize>().unwrap();
    hits[index - 1].clone().clone()
}

fn format_hit(hit: &Hit) -> String {
    format!(
        "{} - {} ({} {})",
        hit.link.type_field, hit.link.link_display, hit.last_price, hit.currency
    )
}
async fn get_scraper_configuration(name: String) {
    println!("Here");
    let symbol = find_symbol(name).await;
    let url = if symbol.link.type_field == SymbolType::STOCK.to_string() {

        format!(
            "https://www.avanza.se/_api/market-guide/stock/{}",
            symbol.link.orderbook_id)
    } else {
        format!(
            "https://www.avanza.se/_api/fund-guide/guide/{}",
            symbol.link.orderbook_id
        )
    };
    let selector = if symbol.link.type_field == SymbolType::STOCK.to_string() {
        "$.quote.last"
    } else {
        "$.nav"
    };
    let config = json!({
        "url": url,
        "selector": selector,

    });
    let mut ctx = ClipboardContext::new().unwrap();
    let res = to_string(&config).expect("Failed to serialize");
    ctx.set_contents(res.clone()).unwrap_or_else( |e| {
        println!("Failed to copy to clipboard: {}", e);
        println!("{}", res);
    });
    println!("Copied to clipboard");
}
