use crate::data_tools;
use crate::models::{Language, RawMapData};
use scraper::{Element, Html, Selector};

static URL_ENGLISH: &str =
    "https://montreal.ca/en/articles/acces-montreal-card-exclusive-offers-and-discounts-5990";
static URL_FRENCH: &str =
    "https://montreal.ca/articles/carte-acces-montreal-offres-et-rabais-exclusifs-5990";

pub async fn parse_for_lang(lang: Language) -> Vec<RawMapData> {
    let old_data: Vec<RawMapData> = data_tools::load_raw_data();
    let mut new_data: Vec<RawMapData> = Vec::new();

    // English
    let html: String = fetch_html(match lang {
        Language::English => URL_ENGLISH,
        Language::French => URL_FRENCH,
    })
    .await
    .expect("Failed to grab data");
    let document: Html = Html::parse_document(&html);

    let title_element_selector: Selector = Selector::parse("p>a>strong").unwrap();
    let title_check_selector: Selector = Selector::parse(":scope>a>strong").unwrap();

    for title_element in document.select(&title_element_selector) {
        let parent: scraper::ElementRef<'_> = title_element
            .parent_element()
            .unwrap()
            .parent_element()
            .unwrap();
        let title: String = parent.inner_html();

        let mut next_elem = parent.next_sibling_element();
        let mut description: String = String::new();

        loop {
            description.push_str(&(next_elem.unwrap().html().to_owned()));
            next_elem = next_elem.unwrap().next_sibling_element();
            if next_elem.is_none()
                || (next_elem.unwrap().select(&title_check_selector).count() == 1)
            {
                break;
            }
        }

        let existing_record_test = old_data.iter().find(|&x| {
            let title_1_fragment = Html::parse_fragment(&title);
            let title_2_fragment = Html::parse_fragment(match lang {
                Language::English => &(x.title_en),
                Language::French => &(x.title_fr),
            });

            let text_selector = Selector::parse("a").unwrap();

            let title_1_elem = title_1_fragment.select(&text_selector).next().unwrap();
            let title_2_elem = title_2_fragment.select(&text_selector).next().unwrap();

            title_1_elem.text().collect::<Vec<_>>() == title_2_elem.text().collect::<Vec<_>>()
        });

        if existing_record_test.is_some() {
            let mut existing_record = existing_record_test.unwrap().clone();
            match lang {
                Language::English => existing_record.description_en = description,
                Language::French => existing_record.description_fr = description,
            }

            new_data.push(existing_record);
            continue;
        }
        new_data.push(match lang {
            Language::English => RawMapData {
                title_en: title,
                title_fr: String::new(),
                description_en: description,
                description_fr: String::new(),
                coordinates: None,
            },
            Language::French => RawMapData {
                title_en: String::new(),
                title_fr: title,
                description_en: String::new(),
                description_fr: description,
                coordinates: None,
            },
        });
    }

    new_data
}

async fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    reqwest::get(url).await?.error_for_status()?.text().await
}
