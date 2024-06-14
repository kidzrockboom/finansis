use dotenv::dotenv;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Deserialize, Serialize, Debug)]
pub struct Company {
    symbol: Option<String>,
    name: Option<String>,
    currency: Option<String>,
    stock_exchange: Option<String>,
    exchange_short_name: Option<String>,
}

pub async fn get_finance_data() -> Result<(), Error> {
    dotenv().ok();

    let path = "data.txt";
    let mut writer = File::create(path).unwrap();

    let request_url = format!(
        "https://financialmodelingprep.com/api/v3/search?query={query}&apikey={apiKey}",
        query = "AA",
        apiKey = std::env::var("FINANCIALMODELINGPREP_API_KEY").expect("Api key goes here")
    );
    println!("{}", request_url);
    let response = reqwest::get(&request_url).await?;

    let companies: Vec<Company> = response.json().await?;
    serde_json::to_writer(&mut writer, &companies).unwrap();
    println!("{:?}", companies);

    todo!("Save data to a data base");
}
