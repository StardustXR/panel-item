use std::env;
use std::path::{Path, PathBuf};

use gluon_codegen::helpers::gen_multiple_modules;
use gluon_codegen::{Derives, ModuleExternalProtocol};

fn main() {
    // Use the codegen library directly to regenerate protocol files
    let output_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("src/protocol");
    gen_multiple_modules(
        &[
            (
                "panel_item",
                Path::new("./schemas/org.stardustxr.item.Panel.gluon"),
            ),
            (
                "panel_item_acceptor",
                Path::new("./schemas/org.stardustxr.item.PanelAcceptor.gluon"),
            ),
        ],
        &[
            &ModuleExternalProtocol {
                rust_module: "stardust_xr_protocol::types",
                external_protocol: stardust_xr_protocol::types::EXTERNAL_PROTOCOL,
            },
            &ModuleExternalProtocol {
                rust_module: "stardust_xr_protocol::keymap",
                external_protocol: stardust_xr_protocol::keymap::EXTERNAL_PROTOCOL,
            },
            &ModuleExternalProtocol {
                rust_module: "stardust_xr_protocol::spatial",
                external_protocol: stardust_xr_protocol::spatial::EXTERNAL_PROTOCOL,
            },
            &ModuleExternalProtocol {
                rust_module: "stardust_xr_protocol::dmatex",
                external_protocol: stardust_xr_protocol::dmatex::EXTERNAL_PROTOCOL,
            },
        ],
        !Derives::DEFAULT,
        &[],
        true,
        output_dir,
    );

    println!("Protocol files regenerated successfully");
}
