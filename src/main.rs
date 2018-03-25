
extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;
use serde_json::{Value};

mod grand_exchange_api;
use grand_exchange_api::request;
use grand_exchange_api::catalogue::items::ApiResult;
use grand_exchange_api::catalogue::items::query;
use grand_exchange_api::catalogue::ItemCategory;

// fn display_result(s: &str) {
//     let responce = reqwest::get(s);

//     match responce {
//         Ok(mut result) => {
//             let v: ApiResult = serde_json::from_str(result.text().unwrap().as_str()).unwrap();
//             println!("{:?}", v.items[0].current.price)
//         },
//         Err(_) => { },
//     }
// }

fn main() {
    println!("Hello1");
    let api: Option<ApiResult> = query(ItemCategory::Familiars, &"c".chars().next().unwrap(), &1);
    println!("Hello2");
    match api {
        Some(result) => { println!("{:?}", result.items[0].name); },
        None => { },
    }
    
    // display_result("http://services.runescape.com/m=itemdb_rs/api/catalogue/items.json?category=9&alpha=c&page=1")
}