use reqwest::StatusCode;
use scraper::{Html, Selector};

mod models;
mod utils;

#[tokio::main]
async fn main() {
    let client = utils::get_client();
    let domain_name = "nyaa.si";
    let query_string = "?f=0&c=0_0&q=dungeon";
    let url = format!("https://{}{}", domain_name, query_string);
    let result = client.get(url).send().await.unwrap();

    let html = match result.status() {
        StatusCode::OK => result.text().await.unwrap(),
        _ => panic!("Something went wrong"),
    };

    let document = Html::parse_document(&html);
    // let result_selector = Selector::parse("tr").unwrap();
    let category_selector = Selector::parse("img.category-icon").unwrap();

    for element in document.select(&category_selector) {
        let category = element.value().attr("alt");
        println!("{:?}", category);
    }
}

