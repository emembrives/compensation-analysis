extern crate leetchi_compensation;

use leetchi_compensation::leetchi;

use rocksdb::DB;

fn main() {
    let database_path = "fundraisings.db";

    let db = DB::open_default(database_path).unwrap();
    let iterator = db.prefix_iterator(b"//details/");
    for (key, value) in iterator {
        let mut parsed_details = match leetchi::schema::FundraisingDetails::from_proto(&value.to_vec()) {
            Ok(d) => d,
            Err(e) => {
                let key_str = String::from_utf8(key.to_vec()).unwrap();
                panic!("Error while reading proto for {:?}: {:?}", &key_str, e);
            }
        };
        if parsed_details.link.len() != 0 {
            continue;
        }
        let link = String::from_utf8(key[10..].to_vec()).unwrap();
        println!("{:?}", link);
        parsed_details.link = link;
        let proto = parsed_details.to_proto().unwrap();
        db.put(&key, &proto).unwrap();
    }
}
