use crate::models::{Language, MapData, RawMapData};

use rocket::serde::json;

// This will embed the JSON data in the executable at compile time:
pub static RAW_DATASET: &str = include_str!("../assets/data.json");

pub fn load_raw_data() -> Vec<RawMapData> {
    json::from_str::<Vec<RawMapData>>(RAW_DATASET).expect("Failed to parse embedded JSON data")
}

pub fn load_for_lang(lang: Language) -> Vec<MapData> {
    load_raw_data()
        .into_iter()
        .map(|item| {
            let (title, description) = match lang {
                Language::English => (item.title_en, item.description_en),
                Language::French => (item.title_fr, item.description_fr),
            };
            MapData {
                title,
                description,
                coordinates: item.coordinates,
            }
        })
        .collect::<Vec<MapData>>()
}
