use std::path::PathBuf;

use serde::Serialize;

// Datum;Konto;Typ av transaktion;Värdepapper/beskrivning;Antal;Kurs;Belopp;Courtage;Valuta;ISIN;Resultat
#[derive(Debug, PartialEq, Serialize)]
enum GhostfolioType {
    BUY,
    DIVIDEND,
    FEE,
    INTEREST,
    ITEM,
    LIABILITY,
    SELL,
    OTHER,
}
impl GhostfolioType {
    pub fn from_avanza(s: &str) -> Self {
        match s {
            "Köp" => Self::BUY,
            "Utdelning" => Self::DIVIDEND,
            "Sälj" => Self::SELL,
            "Utländsk källskatt" => Self::FEE,
            "Övrigt" => Self::OTHER,
            _ => panic!("Unknown ghostfolio type {}", s),
        }
    }
}
fn transform_avanza_number_to_number(s: &str) -> f64 {
    if s == "-" {
        return 0.0;
    }
    let s = s.replace(",", ".");
    s.parse::<f64>().unwrap_or(0.0)
}

#[derive(Debug, Serialize)]
struct Record {
    date: String,
    account: String,
    transaction_type: GhostfolioType,
    security: String,
    amount: f64,
    pricePerUnit: f64,
    price: f64,
    fee: f64,
    currency: String,
    isin: String,
    result: f64,
}
impl Record {
    pub fn from_csv_record(record: csv::StringRecord) -> Self {
        println!("{:?}", record);
        let mut record = Record {
            date: record.get(0).unwrap().to_string(),
            account: record.get(1).unwrap().to_string(),
            transaction_type: GhostfolioType::from_avanza(record.get(2).unwrap()),
            security: record.get(3).unwrap().to_string(),
            amount: transform_avanza_number_to_number(record.get(4).unwrap()),
            pricePerUnit: transform_avanza_number_to_number(record.get(5).unwrap()),
            price: transform_avanza_number_to_number(record.get(6).unwrap()),
            fee: transform_avanza_number_to_number(record.get(7).unwrap()),
            currency: record.get(8).unwrap().to_string(),
            isin: record.get(9).unwrap().to_string(),
            result: transform_avanza_number_to_number(record.get(10).unwrap()),
        };
        if record.transaction_type == GhostfolioType::OTHER {
            println!("{:?}", record);
            if record.amount == 0.0 {
                // Probably dividend, We have two entries one where amount positive and one where
                // ngative
                if record.amount > 0.0 {
                    record.transaction_type = GhostfolioType::BUY;
                } else {
                    record.transaction_type = GhostfolioType::SELL;
                    record.amount = record.amount.abs();
                }
            } else if record.price < 0.0 {
                // Probably fee
                record.transaction_type = GhostfolioType::FEE;
            } else if record.price > 0.0 {
                // INTEREST
                record.transaction_type = GhostfolioType::INTEREST;
            }
        }
        record
    }
}

pub fn parse_from_file(path: PathBuf) -> () {
    let skip_types = vec!["Insättning", "Uttag"];
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_path(path)
        .expect("Failed to read csv file");
    let mut parsed = Vec::new();
    for result in rdr.records() {
        if let Ok(record) = result {
            if skip_types.contains(&record.get(2).unwrap()) {
                continue;
            }
            let parsed_record = Record::from_csv_record(record);
            parsed.push(parsed_record);
        }
    }
    println!("{:?}", parsed);
}
