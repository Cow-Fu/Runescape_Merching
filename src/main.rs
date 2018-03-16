
extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;
use serde_json::{Value, Map};

mod grand_exchange_api;
use grand_exchange_api::catalogue::ItemCategory;

#[derive(Debug, Serialize, Deserialize)]
struct ApiResult {
    items: Vec<Map<String, Value>>,
    total: usize

}

fn display_result(s: &str) {
    let responce =  reqwest::get(s);
    match responce {
        Ok(mut result) => {
            let v: ApiResult = serde_json::from_str(result.text().unwrap().as_str()).unwrap();
            println!("{:?}", v.items[0]["icon"].as_str().unwrap())
        },
        Err(_) => { },
    }
}

fn main() {
    // let responce = reqwest::get("http://services.runescape.com/m=itemdb_rs/1520937642029_obj_big.gif?id=12091");
    display_result("http://services.runescape.com/m=itemdb_rs/api/catalogue/items.json?category=9&alpha=c&page=1")
}