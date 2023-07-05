use glob::glob;
extern crate protoc_rust;

fn main() {
    protoc_rust::Codegen::new()
        .includes([
            concat!(env!("CARGO_MANIFEST_DIR"), "/../../../protos"),
            "/opt/include",
        ])
        .inputs(
            glob(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../../protos/*.proto"
            ))
            .expect("invalid protobuf inputs!")
            .flatten(),
        )
        .out_dir("../src")
        .customize(protoc_rust::Customize {
            gen_mod_rs: Some(true),
            ..Default::default()
        })
        .run()
        .expect("Running protoc failed.");
}
