use std::env;
use std::path::PathBuf;

use gluon_codegen_rust::Derives;
use gluon_codegen_rust::helpers::gen_single_module;

fn main() {
    // Watch for changes to KDL schema files

    // Use the codegen library directly to regenerate protocol files
    let output_file =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("src/protocol.rs");
    gen_single_module(
        "./schemas/org.stardustxr.item.Panel.gluon",
        !Derives::DEFAULT,
        output_file,
    );

    println!("Protocol files regenerated successfully");
}
