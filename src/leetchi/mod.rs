mod index_parser;
mod details_parser;
mod schema;

pub fn get_all_fundraisings() -> Result<index_parser::IndexPageResult, index_parser::IndexPageError> {
    let mut i: i32 = 1;
    let mut v: Vec<schema::FundraisingCardSummary> = Vec::new();
    loop {
        let url = format!("https://www.leetchi.com/fr/cagnottes?p={}", i);
        match index_parser::get_one_index_page(&url) {
            Err(e) => {
                match e {
                    index_parser::IndexPageError::NoResult => break,
                    index_parser::IndexPageError::HtmlParsingError(_) => return Err(e),
                    index_parser::IndexPageError::RequestError(_) => return Err(e),
                }
            },
            Ok(mut r) => {
                v.append(&mut r.fundraisings);
                i = i + 1;
            }
        }
    }
    Ok(index_parser::IndexPageResult{fundraisings: v})
}

pub fn get_details(summary: &schema::FundraisingCardSummary) -> Result<schema::FundraisingDetail, details_parser::DetailPageError> {
    details_parser::get_details_page("https://www.leetchi.com/", summary)
}