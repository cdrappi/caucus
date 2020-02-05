extern crate prost_build;

fn main() {
    prost_build::compile_protos(&["user.proto"], &["../../protos/"])
        .expect("Failed to compile user proto");
}
