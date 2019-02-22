#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocksdb::DB;
use clap::{App, Arg};
use common::schema::FundraisingDetails;
use rocket::response::status::Custom;
use rocket::http::Status;
use std::fmt;
use rocket_contrib::json::Json;

struct FundraisingDb {
    db: DB,
}

impl FundraisingDb {
    pub fn new(database_path: &str) -> FundraisingDb {
        FundraisingDb{
            db: DB::open_default(database_path).unwrap()
        }
    }

    pub fn get_unevaled_fundraising(&self) -> Result<Option<FundraisingDetails>, common::schema::FromProtoError> {
        let mut details_iterator : rocksdb::DBRawIterator = self.db.prefix_iterator(b"//details/").into();
        details_iterator.seek_to_first();
        let mut eval_iterator : rocksdb::DBRawIterator = self.db.prefix_iterator(b"//eval/").into();
        eval_iterator.seek_to_last();
        if eval_iterator.valid() {
            let mut last_eval_link = eval_iterator.key().unwrap()[7..].to_vec();
            let mut details_key = b"//details/".to_vec();
            details_key.append(&mut last_eval_link);
            details_iterator.seek(&details_key);
            details_iterator.next();
        }
        if !details_iterator.valid() {
            // We have evaluated everything.
            return Ok(None);
        }
        match common::schema::FundraisingDetails::from_proto(&details_iterator.value().unwrap().to_vec()) {
            Ok(p) => return Ok(Some(p)),
            Err(e) => return Err(e),
        }
    }
}

#[get("/next")]
fn next(db: rocket::State<FundraisingDb>) -> Result<Option<Json<FundraisingDetails>>, Custom<String>> {
    match db.get_unevaled_fundraising() {
        Ok(r) => match r {
            Some(details) => Ok(Some(Json(details))),
            None => Ok(None),
        },
        Err(e) => Err(Custom(Status::InternalServerError, format!("{:#?}", &e))),
    }
}

#[post("/save", data = "<eval>")]
fn save(db: rocket::State<FundraisingDb>, eval: Json<String>) -> &'static str {
    "Hello, world!"
}

fn main() {
    let matches = App::new("Leetchi analyzer")
        .version("0.1")
        .author("Etienne J. Membrives <etienne@membrives.fr>")
        .about("Webserver for offline analysis.")
        .arg(
            Arg::with_name("database_path")
                .help("Sets the database path to use")
                .required(true),
        )
        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let database_path = matches
        .value_of("database_path")
        .unwrap_or("fundraisings.db");

    rocket::ignite()
        .manage(FundraisingDb::new(database_path))
        .mount("/", routes![next, save])
        .launch();
}