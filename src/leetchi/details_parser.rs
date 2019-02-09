extern crate reqwest;
extern crate scraper;
extern crate itertools;

use crate::leetchi::schema;
use scraper::{Html, Selector};
use reqwest::header;


#[derive(Debug)]
pub enum DetailPageError {
    RequestError(reqwest::Error),
    HtmlParsingError(&'static str),
    NoResult,
}

fn select<'a>(
    fragment: &'a scraper::ElementRef,
    selector_str: &str,
) -> Result<String, DetailPageError> {
    let selector = Selector::parse(selector_str).unwrap();
    let text = match fragment.select(&selector).last() {
        None => return Err(DetailPageError::HtmlParsingError("unable to find link node")),
        Some(node) => node.text().fold(String::new(), |acc, a| acc + a),
    };
    Ok(text)
}

fn parse_detail_page(text: &str) -> Result<schema::FundraisingDetail, DetailPageError> {
    let selector = Selector::parse("body").unwrap();
    let html = Html::parse_document(text);
    let fragment = match html.select(&selector).last() {
        Some(f) => f,
        None => return Err(DetailPageError::HtmlParsingError("No body in page")),
    };
    let title = match select(&fragment, ".page-heading") {
        Err(e) => return Err(e),
        Ok(t) => t,
    };
    Err(DetailPageError::NoResult)
}

pub fn get_details_page(base_url: &str, summary: &schema::FundraisingCardSummary) -> Result<schema::FundraisingDetail, DetailPageError> {
    let client = reqwest::Client::new();
    let request = client.get(&(base_url.to_owned() + &summary.link)).
        header(header::USER_AGENT, header::HeaderValue::from_static(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/60.0.3112.113 Safari/537.36"));
    let mut res = match request.send() {
        Ok(response) => response,
        Err(e) => return Err(DetailPageError::RequestError(e)),
    };

    let text = match res.text() {
        Err(e) => return Err(DetailPageError::RequestError(e)),
        Ok(text) => text,
    };
    parse_detail_page(&text)
}
