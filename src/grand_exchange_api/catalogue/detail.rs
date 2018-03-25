

use serde_json::Value;

static BASE_URL: &str = "http://services.runescape.com/m=itemdb_rs/api/catalogue/detail.json?item=X";

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemDetail {
    pub icon: String,
    pub icon_large: String,
    pub id: i32,
    #[serde(rename = "type")] 
    pub item_type: String,
    #[serde(rename = "typeIcon")]
    pub item_type_icon: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ItemTrend {
    pub trend: String, 
    pub price: Value
}
