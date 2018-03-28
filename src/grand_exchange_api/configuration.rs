
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use grand_exchange_api::request;

static BASE_URL: &str = "http://secure.runescape.com/m=itemdb_rs/api/info.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigInfo {
    lastConfigUpdateRuneday: i32
}

pub fn get_last_update() -> Option<ConfigInfo> {
    return request::<ConfigInfo>(BASE_URL);
}