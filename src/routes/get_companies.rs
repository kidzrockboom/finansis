use axum::{debug_handler, http::StatusCode, Json};
use std::{fs::File, io::BufReader};
use tracing_subscriber::field::debug;

use crate::api::finance_data::Company;

fn read_json() -> Vec<Company> {
    // Replace this with database call
    let path = "data.txt";

    let file = File::open(path).expect("Cannot read file");
    let reader = BufReader::new(file);

    let companies: Vec<Company> =
        serde_json::from_reader(reader).expect("Cannot parse json from file");

    companies
}

pub async fn get_companies() -> (StatusCode, Json<Vec<Company>>) {
    let companies = read_json();

    (StatusCode::OK, Json(companies))
}
