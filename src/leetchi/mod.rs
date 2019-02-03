mod parser;
mod schema;

extern crate capnp;

pub mod proto_capnp {
  include!(concat!(env!("OUT_DIR"), "/proto_capnp.rs"));
}

pub fn get_all_fundraisings() -> Result<parser::IndexPageResult, parser::IndexPageError> {
    let mut i: i32 = 1;
    let mut v: Vec<schema::FundraisingCardSummary> = Vec::new();
    loop {
        let url = format!("https://www.leetchi.com/fr/cagnottes/medical?p={}", i);
        match parser::get_one_index_page(&url) {
            Err(e) => {
                match e {
                    parser::IndexPageError::NoResult => break,
                    parser::IndexPageError::HtmlParsingError(_) => return Err(e),
                    parser::IndexPageError::RequestError(_) => return Err(e),
                }
            },
            Ok(mut r) => {
                v.append(&mut r.fundraisings);
                i = i + 1;
            }
        }
    }
    Ok(parser::IndexPageResult{fundraisings: v})
}