use sigstore_protobuf_specs::dev::sigstore::{
    bundle::v1::{bundle::Content, Bundle},
    common::v1::MessageSignature,
};

#[test]
fn bundle() {
    let bundle_json = std::fs::read_to_string("tests/bundle.json").unwrap();

    let deserialize = serde_json::from_str::<Bundle>(&bundle_json);
    println!("{:?}", deserialize);
    assert!(deserialize.is_ok());
}
