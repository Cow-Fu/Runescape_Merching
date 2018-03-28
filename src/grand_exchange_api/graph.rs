
extern crate reqwest;
extern crate serde;
extern crate serde_json;


use std::collections::HashMap;
use grand_exchange_api::request;

use serde_json::{Value, Map};

static BASE_URL: &str = "http://services.runescape.com/m=itemdb_rs/api/graph/";

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemGraph {
    pub daily: HashMap<String, i64>,
    pub average: HashMap<String, i64>
}

pub fn get_item_graph(id: i32) -> Option<ItemGraph> {
    return request::<ItemGraph>(&format!("{}{}{}", BASE_URL, id, ".json"));
}