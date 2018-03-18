
extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigInfo {
    lastConfigUpdateRuneday: i32
}

pub fn get_last_update() -> Option<ConfigInfo> {
    let responce = reqwest::get("http://secure.runescape.com/m=itemdb_rs/api/info.json");
    match responce {
        Ok(mut result) => {
            let v: ConfigInfo = serde_json::from_str(result.text().unwrap().as_str()).unwrap();
            // println!("{:?}", v.lastConfigUpdateRuneday);
            return Some(v);
        },
        Err(_) => { println!("Hi"); },
    }
    return None;
}