use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::api::finance_data::Company;

fn read_json() -> Json<Company> {
    todo!("Replace this with database call");
    let companies: Vec<Company> = serde_json
}

pub fn get_companies() -> (StatusCode, Json<Company>) {
    
}
