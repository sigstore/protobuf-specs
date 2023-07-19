fn main() -> anyhow::Result<()> {
    prost_build::Config::new()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute(".", r#"#[serde(rename_all = "camelCase")]"#)
        .compile_well_known_types()
        .extern_path(".google.protobuf", "::pbjson_types")
        .out_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/../src/codegen"))
        .compile_protos(
            &glob::glob(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../../protos/*.proto"
            ))
            .expect("no protos found!")
            .flatten()
            .collect::<Vec<_>>(),
            &[
                concat!(env!("CARGO_MANIFEST_DIR"), "/../../../protos"),
                "/opt/include",
            ],
        )?;

    pbjson_build::Builder::new()
        .out_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/../src/codegen"))
        .build(&[""])?;

    Ok(())
}
