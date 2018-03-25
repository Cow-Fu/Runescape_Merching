
use serde_json::Value;
use grand_exchange_api::request;
use grand_exchange_api::catalogue::ItemCategory;


static BASE_URL: &str = "http://services.runescape.com/m=itemdb_rs/api/catalogue/items.json?";


#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResult {
    pub items: Vec<ItemInfo>,
    pub total: usize
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemInfo {
    pub icon: String,
    pub icon_large: String,
    pub id: i32,
    #[serde(rename = "type")] 
    pub item_type: String,
    #[serde(rename = "typeIcon")]
    pub item_type_icon: String,
    pub name: String,
    pub description: String,
    pub current: ItemTrend,
    pub today: ItemTrend
 }

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemTrend {
    pub trend: String, 
    pub price: Value
}


pub fn query(category: ItemCategory, alpha: &char, pgNumber: &u8) -> Option<ApiResult> {
    let cat = ItemCategory::from_category(category).unwrap().to_string();
    let url = format!("{}category={}&alpha={}&page={}", BASE_URL, cat, alpha, pgNumber);
    // let url = BASE_URL.to_owned() + s.as_str();
    return request::<ApiResult>(&url);
}
