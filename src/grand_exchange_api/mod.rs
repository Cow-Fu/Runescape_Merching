

pub mod catalogue;
pub mod configuration;
pub mod graph;

extern crate reqwest;
extern crate serde;
extern crate serde_json;

pub fn request<T>(url: &str) -> Option<T> where T: serde::de::DeserializeOwned {
    let responce = reqwest::get(url);
    match responce {
        Ok(mut result) => {
            let v: T = serde_json::from_str(result.text().unwrap().as_str()).unwrap();
            return Some(v);
        },
        Err(_) => { },
    }
    return None;
}