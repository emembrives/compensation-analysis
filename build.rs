extern crate capnpc;

fn main() {
    capnpc::CompilerCommand::new()
        .src_prefix("proto")
        .file("proto/proto.capnp")
        .run().expect("schema compiler command");
}
