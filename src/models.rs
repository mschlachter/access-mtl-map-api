use rocket::request::FromParam;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RawMapData {
    pub title_en: String,
    pub title_fr: String,
    pub description_en: String,
    pub description_fr: String,
    pub coordinates: Option<(f64, f64)>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapData {
    pub title: String,
    pub description: String,
    pub coordinates: Option<(f64, f64)>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum Language {
    English,
    French,
}

impl<'r> FromParam<'r> for Language {
    type Error = &'r str;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        match param {
            "en" => Ok(Language::English),
            "fr" => Ok(Language::French),
            other => Err(other),
        }
    }
}
