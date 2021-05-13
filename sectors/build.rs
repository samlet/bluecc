extern crate capnpc;

use std::env;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    println!("output dir: {:?}", out_dir);

    ::capnpc::CompilerCommand::new()
        .file("protocols/pubsub.capnp")
        .file("protocols/addressbook.capnp")
        .run().unwrap();
}

