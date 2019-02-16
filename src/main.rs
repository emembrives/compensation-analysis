#[macro_use]
extern crate lazy_static;
extern crate clap;

mod leetchi;
mod protos;

use clap::{App, Arg};
use rocksdb::DB;
use std::{thread, time};

fn summaries(database_path: &str) {
    let db = DB::open_default(database_path).unwrap();

    let results = match leetchi::get_all_fundraisings() {
        Err(e) => panic!(e),
        Ok(r) => r,
    };

    for summary in results.fundraisings {
        match summary.to_proto() {
            Err(e) => panic!(e),
            Ok(p) => {
                db.put(&("//summary/".to_owned() + &summary.link).into_bytes(), &p)
                    .unwrap();
            }
        }
    }
}

fn details(database_path: &str) {
    let db = DB::open_default(database_path).unwrap();
    let iterator = db.prefix_iterator(b"//summary/");
    let mut prev = time::Instant::now();
    for (_, value) in iterator {
        let summary = leetchi::schema::FundraisingCardSummary::from_proto(&value.to_vec()).unwrap();
        let downloaded_details = db.get(&("//summary/".to_owned() + &summary.link).into_bytes()).unwrap();
        if downloaded_details.is_some() {
            println!("Details known for {}", &summary.link);
            if leetchi::schema::FundraisingDetails::from_proto(&downloaded_details.unwrap().to_vec()).is_ok() {
                continue;
            }
            println!("Unable to parse {}, downloading again", &summary.link);
        }

        println!("Downloading details for {}", &summary.link);
        let now = time::Instant::now();
        if now - prev < time::Duration::from_secs(1) {
            thread::sleep(time::Duration::from_secs(1) - (now - prev));
        }
        prev = now;
        let details = leetchi::get_details(&summary).unwrap();
        let details_proto = details.to_proto().unwrap();
        db.put(
            &("//details/".to_owned() + &summary.link).into_bytes(),
            &details_proto,
        )
        .unwrap();
    }
}

fn main() {
    let matches = App::new("Leetchi parser")
        .version("0.1")
        .author("Etienne J. Membrives <etienne@membrives.fr>")
        .about("Parses Leetchi pages for offline analysis.")
        .arg(
            Arg::with_name("database_path")
                .help("Sets the database path to use")
                .required(true),
        )
        .arg(
            Arg::with_name("command")
                .help("Command to run")
                .required(true)
                .possible_value("summaries")
                .possible_value("details"),
        )
        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let database_path = matches
        .value_of("database_path")
        .unwrap_or("fundraisings.db");

    match matches.value_of("command") {
        Some("summaries") => summaries(database_path),
        Some("details") => details(database_path),
        Some(_) => panic!("Unknown command"),
        None => panic!("Please select a subcommand"),
    }
}
