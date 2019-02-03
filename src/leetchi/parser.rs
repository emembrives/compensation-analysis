extern crate reqwest;
extern crate scraper;

use crate::leetchi::schema;
use scraper::{Html, Selector};

pub struct IndexPageResult {
    pub fundraisings: Vec<schema::FundraisingCardSummary>,
}

#[derive(Debug)]
pub enum IndexPageError {
    RequestError(reqwest::Error),
    HtmlParsingError(&'static str),
    NoResult,
}

fn select<'a>(
    fragment: &'a scraper::ElementRef,
    selector_str: &str,
) -> Result<&'a str, IndexPageError> {
    let selector = Selector::parse(selector_str).unwrap();
    let text = match fragment.select(&selector).last() {
        None => return Err(IndexPageError::HtmlParsingError("unable to find link node")),
        Some(node) => match node.text().last() {
            None => return Err(IndexPageError::HtmlParsingError("unable to find link text")),
            Some(t) => t,
        },
    };
    Ok(text)
}

fn parse_index_page(text: &str) -> Result<IndexPageResult, IndexPageError> {
    let fragment = Html::parse_document(text);
    let has_result_selector = Selector::parse(".section-no-result").unwrap();
    if fragment.select(&has_result_selector).count() != 0 {
        return Err(IndexPageError::NoResult);
    }
    let selector = Selector::parse(".fundraising-card").unwrap();
    let fundraisings = fragment.select(&selector);
    let mut summaries: Vec<schema::FundraisingCardSummary> = Vec::new();
    for fundraising in fundraisings {
        let link_selector = Selector::parse("a.fundraising-card__link").unwrap();
        let link = match fundraising.select(&link_selector).last() {
            None => return Err(IndexPageError::HtmlParsingError("unable to find link node")),
            Some(node) => match node.value().attr("href") {
                None => return Err(IndexPageError::HtmlParsingError("unable to find link text")),
                Some(t) => t,
            },
        };
        let title = match select(&fundraising, ".fundraising-card__heading") {
            Err(e) => return Err(e),
            Ok(t) => t,
        };
        let description = match select(&fundraising, ".fundraising-card__description") {
            Err(e) => return Err(e),
            Ok(t) => t,
        };
        let corner_selector = Selector::parse(".corner.corner-lg.leetchi").unwrap();
        let verified = match fundraising.select(&corner_selector).last() {
            None => false,
            Some(_) => true,
        };
        let contributors = match select(&fundraising, ".stat-contributors strong") {
            Err(e) => return Err(e),
            Ok(s) => match u32::from_str_radix(s.trim(), 10) {
                Err(_) => {
                    println!("Unable to parse contributors for {}, {}", s.trim(), link);
                    return Err(IndexPageError::HtmlParsingError(
                        "Unable to parse contributors",
                    ));
                }
                Ok(i) => i,
            },
        };
        // Not parsing amount
        summaries.push(schema::FundraisingCardSummary::new(
            link,
            title,
            description,
            verified,
            contributors,
        ));
    }
    Ok(IndexPageResult {
        fundraisings: summaries,
    })
}

pub fn get_one_index_page(url: &str) -> Result<IndexPageResult, IndexPageError> {
    let client = reqwest::Client::new();
    let request = client.get(url);
    let mut res = match request.send() {
        Ok(response) => response,
        Err(e) => return Err(IndexPageError::RequestError(e)),
    };

    let text = match res.text() {
        Err(e) => return Err(IndexPageError::RequestError(e)),
        Ok(text) => text,
    };
    parse_index_page(&text)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use std::fs;

    #[test]
    fn test_parse_index() {
        let contents = fs::read_to_string("golden/index.html").expect("Unable to read golden file");
        let result = parse_index_page(&contents);
        match result {
            Err(e) => {
                println!("{:#?}", e);
                assert!(false);
            }
            Ok(r) => {
                let index_page_results = r;
                println!("{}", index_page_results.fundraisings.len());
                assert_eq!(index_page_results.fundraisings.len(), 24);
            }
        }
    }

    #[test]
    fn test_parse_index_no_result() {
        let contents =
            fs::read_to_string("golden/no_result.html").expect("Unable to read golden file");
        let result = parse_index_page(&contents);
        match result {
            Err(e) => {
                match e {
                    IndexPageError::NoResult => return,
                    IndexPageError::RequestError(_) => assert!(false),
                    IndexPageError::HtmlParsingError(_) => assert!(false),
                }
            }
            Ok(_) => assert!(false),
        }
    }
}
