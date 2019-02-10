extern crate reqwest;
extern crate scraper;
extern crate itertools;
extern crate regex;

use crate::leetchi::schema;
use scraper::{Html, Selector};
use reqwest::header;
use regex::Regex;


#[derive(Debug)]
pub enum DetailPageError {
    RequestError(reqwest::Error),
    HtmlParsingError(&'static str),
    IntParsingError(std::num::ParseIntError),
}

fn select<'a>(
    fragment: &'a scraper::ElementRef,
    selector_str: &str,
) -> Result<String, DetailPageError> {
    let selector = Selector::parse(selector_str).unwrap();
    let text = match fragment.select(&selector).last() {
        None => return Err(DetailPageError::HtmlParsingError("unable to find link node")),
        Some(node) => node.text().fold(String::new(), |acc, a| acc + " " + a),
    };
    Ok(text.trim().to_owned())
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
    let trimmed_title = title.replace("\n", "");
    let description = match select(&fragment, ".fdr-description") {
        Err(e) => return Err(e),
        Ok(t) => t,
    };
    let corner_selector = Selector::parse(".corner.corner-lg.leetchi").unwrap();
    let verified = match fragment.select(&corner_selector).last() {
        None => false,
        Some(_) => true,
    };
    let collected = match select(&fragment, "header.o-article-status__header.c-header") {
        Err(_) => None,
        Ok(t) => Some(t),
    };
    let contributors_str = match select(&fragment, ".c-status__counter") {
        Err(e) => return Err(e),
        Ok(t) => t,
    };
    let contributors = match u32::from_str_radix(contributors_str.replace('"', "").trim(), 10) {
        Err(e) => return Err(DetailPageError::IntParsingError(e)),
        Ok(n) => n
    };
    let fundraiser_str = match select(&fragment, ".panel-fundraiser div.panel-body") {
        Err(e) => return Err(e),
        Ok(t) => t,
    };
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\s+").unwrap();
    }
    let fundraiser = RE.replace_all(&fundraiser_str, " ");
    let mut result = schema::FundraisingDetail::new(trimmed_title, description, verified, collected, contributors, fundraiser.to_string());

    let label_geoloc_selector = Selector::parse("#stickySidebar .label-lists a.label-geolocation").unwrap();
    let labels_geoloc = fragment.select(&label_geoloc_selector);
    for label_geoloc in labels_geoloc {
        let text = label_geoloc.text().fold(String::new(), |acc, a| acc + " " + a).trim().to_owned();
        result.tags.push(schema::Label{
            name: text,
            label_type: schema::LabelType::Location,
        });
    }

    let label_type_selector = Selector::parse("#stickySidebar .label-lists a.label-event-type").unwrap();
    let labels_type = fragment.select(&label_type_selector);
    for label_type in labels_type {
        let text = label_type.text().fold(String::new(), |acc, a| acc + " " + a).trim().to_owned();
        result.tags.push(schema::Label{
            name: text,
            label_type: schema::LabelType::EventType,
        });
    }
    Ok(result)
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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use std::fs;

    #[test]
    fn test_parse_regular() {
        let contents = fs::read_to_string("golden/fundraising_regular.html").expect("Unable to read golden file");
        let result = parse_detail_page(&contents);
        match result {
            Err(e) => {
                println!("{:#?}", e);
                assert!(false);
            }
            Ok(details) => {
                println!("{:#?}", &details);
                assert_eq!(details.title, "Une voiture aménagée pour Marianne");
                assert_ne!(details.description.len(), 0);
                assert!(!details.verified);
                assert!(details.collected.is_some());
                assert_eq!(details.contributors, 80);
                assert_eq!(details.fundraiser, "Marianne Grafteaux");
                assert_eq!(details.tags.len(), 4);
            }
        }
    }

    #[test]
    fn test_parse_verified() {
        let contents = fs::read_to_string("golden/fundraising_verified.html").expect("Unable to read golden file");
        let result = parse_detail_page(&contents);
        match result {
            Err(e) => {
                println!("{:#?}", e);
                assert!(false);
            }
            Ok(details) => {
                println!("{:#?}", &details);
                assert_eq!(details.title, "Les pas d'une princesse");
                assert_ne!(details.description.len(), 0);
                assert!(details.verified);
                assert!(details.collected.is_some());
                assert_eq!(details.contributors, 162);
                assert_eq!(details.fundraiser, "Maud goasduff");
                assert_eq!(details.tags.len(), 4);
            }
        }
    }

    #[test]
    fn test_parse_no_amount() {
        let contents = fs::read_to_string("golden/fundraising_no_amount.html").expect("Unable to read golden file");
        let result = parse_detail_page(&contents);
        match result {
            Err(e) => {
                println!("{:#?}", e);
                assert!(false);
            }
            Ok(details) => {
                println!("{:#?}", &details);
                assert_eq!(details.title, "Pour Tom");
                assert_ne!(details.description.len(), 0);
                assert!(!details.verified);
                assert!(details.collected.is_none());
                assert_eq!(details.contributors, 276);
                assert_eq!(details.fundraiser, "Michel SEIGLE-VATTE");
                assert_eq!(details.tags.len(), 1);
            }
        }
    }
}
