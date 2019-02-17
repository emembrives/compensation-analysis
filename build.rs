extern crate protoc_rust;

use protoc_rust::Customize;

fn main() {
	println!("cargo:rerun-if-changed=protos/*");
	protoc_rust::run(protoc_rust::Args {
	    out_dir: "src/protos",
	    input: &["protos/fundraising.proto"],
	    includes: &["protos"],
	    customize: Customize {
	      ..Default::default()
	    },
	}).expect("protoc");
}
