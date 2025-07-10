// method to return a list of ETFs from a CSV file
use csv::ReaderBuilder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Etf {
    pub symbol: String,
    pub name: String,
    // pub currency: String,
    // pub summary: String,
    // pub category_group: String,
    // pub category: String,
    // pub family: String,
    // pub exchange: String,
}

pub fn read_etfs_from_csv(file_path: &str) -> Result<Vec<Etf>, Box<dyn std::error::Error>> {
    let mut rdr = ReaderBuilder::new().from_path(file_path)?;
    let mut etfs = Vec::new();

    for result in rdr.deserialize() {
        let etf: Etf = result?;

        // skip if the symbol is empty or contains spaces
        if etf.symbol.trim().is_empty() || etf.symbol.contains(' ') {
            continue;
        }

        // skip if the symbol starts with ^
        if etf.symbol.starts_with('^') {
            continue;
        }

        etfs.push(etf);
    }

    Ok(etfs)
}
