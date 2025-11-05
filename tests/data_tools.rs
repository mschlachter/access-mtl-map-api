extern crate access_mtl_map_api;

use access_mtl_map_api::{data_tools, models::Language};

#[test]
fn load_raw_and_lang_transforms() {
    let raw = data_tools::load_raw_data();
    assert!(!raw.is_empty(), "expected assets/data.json to contain at least one item");

    let first_raw = &raw[0];

    // Ensure English mapping returns the english title/description
    let en = data_tools::load_for_lang(Language::English);
    assert!(!en.is_empty());
    assert_eq!(en[0].title, first_raw.title_en);
    assert_eq!(en[0].description, first_raw.description_en);

    // Ensure French mapping returns the french title/description
    let fr = data_tools::load_for_lang(Language::French);
    assert!(!fr.is_empty());
    assert_eq!(fr[0].title, first_raw.title_fr);
    assert_eq!(fr[0].description, first_raw.description_fr);

    // coordinates forwarded through
    assert_eq!(en[0].coordinates, first_raw.coordinates);
    assert_eq!(fr[0].coordinates, first_raw.coordinates);
}
