#[macro_use] extern crate lazy_static;
extern crate capnp;

mod leetchi;

use rocksdb::DB;

pub mod fundraising_capnp {
  include!(concat!(env!("OUT_DIR"), "/fundraising_capnp.rs"));
}

fn main() {
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
        match leetchi::get_details(&summary) {
            Err(e) => panic!(e),
            Ok(_) => {}
        }
    }
}
