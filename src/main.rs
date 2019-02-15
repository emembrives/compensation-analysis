#[macro_use] extern crate lazy_static;

mod leetchi;
mod protos;

use rocksdb::DB;
use std::{thread, time};

fn summaries() {
    let db = DB::open_default("fundraisings.db").unwrap();

    let results = match leetchi::get_all_fundraisings() {
        Err(e) => panic!(e),
        Ok(r) => r,
    };

    for summary in results.fundraisings {
        match summary.to_proto() {
            Err(e) => panic!(e),
            Ok(p) => {
                db.put(&("//summary/".to_owned() + &summary.link).into_bytes(), &p).unwrap();
            }
        }
    }
}

fn details() {
    let db = DB::open_default("fundraisings.db").unwrap();
    let iterator = db.prefix_iterator(b"//summary/");
    let mut prev = time::Instant::now();
    for (_, value) in iterator {
        let summary = leetchi::schema::FundraisingCardSummary::from_proto(&value.to_vec()).unwrap();
        println!("Downloading details for {}", &summary.link);
        let now = time::Instant::now();
        if now - prev < time::Duration::from_secs(1) {
            thread::sleep(time::Duration::from_secs(1) - (now - prev));
        }
        prev = now;
        let details = leetchi::get_details(&summary).unwrap();
        let details_proto = details.to_proto().unwrap();
        db.put(&("//details/".to_owned() + &summary.link).into_bytes(), &details_proto).unwrap();
    }
}

fn main() {
    summaries();
    details();
}
