#[macro_use]
extern crate rocket;

use access_mtl_map_api::data_tools;
use access_mtl_map_api::models::{Language, MapData};

use rocket::serde::json::Json;
use rocket::response::content;

#[get("/")]
fn index() -> &'static str {
    "Small API to fetch the list of Access Montreal partners with geo-coordinate data added. Can be used to build a map of places to use an Access Montreal card, e.g. https://www.schlachter.xyz/projects/access-montreal-map

Contains data scraped from https://montreal.ca/en/articles/acces-montreal-card-exclusive-offers-and-discounts-5990 and https://montreal.ca/articles/carte-acces-montreal-offres-et-rabais-exclusifs-5990

Endpoints
- GET /data/raw - returns the raw JSON records
- GET /data/<lang> - returns localized data for language `en` or `fr` (e.g. `/data/en`)"
}

#[get("/data/raw")]
fn get_raw_data() -> content::RawJson<&'static str> {
    content::RawJson(data_tools::RAW_DATASET)
}

#[get("/data/<lang>")]
fn get_data_for_language(lang: Language) -> Json<Vec<MapData>> {
    Json(data_tools::load_for_lang(lang))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_raw_data, get_data_for_language])
}
