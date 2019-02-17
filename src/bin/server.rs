#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate leetchi_compensation;

use leetchi_compensation::leetchi;
use rocksdb::DB;
use clap::{App, Arg};

struct FundraisingDb {
    db: DB,
}

impl FundraisingDb {
    pub fn new(database_path: &str) -> FundraisingDb {
        let db = DB::open_default(database_path).unwrap();
        FundraisingDb{db: db}
    }

    pub fn find_next_to_eval(&self) {

    }
}

#[get("/next")]
fn next(db: rocket::State<FundraisingDb>) -> &'static str {
    db.find_next_to_eval();
    "foo"
}

#[get("/record")]
fn record(db: rocket::State<FundraisingDb>) -> &'static str {
    "Hello, world!"
}


fn main() {
        let matches = App::new("Leetchi parser")
        .version("0.1")
        .author("Etienne J. Membrives <etienne@membrives.fr>")
        .about("Server for manual annotation of Leetchi data.")
        .arg(
            Arg::with_name("database_path")
                .help("Sets the database path to use")
                .required(true),
        )
        .get_matches();


    let database_path = matches
        .value_of("database_path")
        .unwrap_or("fundraisings.db");

    rocket::ignite()
        .manage(FundraisingDb::new(database_path))
        .mount("/", routes![next, record]).launch();
}