use sigstore_protobuf_specs::dev::sigstore::common::v1::HashOutput;

#[test]
fn serialize_hash_output() {
    assert_eq!(
        serde_json::to_string(&HashOutput {
            algorithm: 0i32,
            digest: vec![1u8, 2u8, 3u8],
        })
        .unwrap(),
        r#"{"algorithm":0,"digest":[1,2,3]}"#
    )
}

#[test]
fn deserialize_hash_output() {
    assert_eq!(
        serde_json::from_str::<HashOutput>(r#"{"algorithm":0,"digest":[1,2,3]}"#).unwrap(),
        HashOutput {
            algorithm: 0i32,
            digest: vec![1u8, 2u8, 3u8]
        }
    )
}
