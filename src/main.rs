mod leetchi;

use rocksdb::DB;

fn main() {
    let db = DB::open_default("summaries.db").unwrap();

    let results = match leetchi::get_all_fundraisings() {
        Err(e) => panic!(e),
        Ok(r) => r,
    };

    for summary in results.fundraisings {
        match summary.to_proto() {
            Err(e) => panic!(e),
            Ok(p) => {
                db.put(&summary.link.into_bytes(), &p).unwrap();
            }
        }
    }
}
