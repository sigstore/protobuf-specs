use sigstore_protobuf_specs::dev::sigstore::common::v1::{HashOutput, LogId, MessageSignature};

/// HashOutput, a structure using only primitive types
#[test]
fn primitives() {
    let hash_output_json = r#"{"algorithm":0,"digest":[1,2,3]}"#;
    let hash_output_rs = HashOutput {
        algorithm: 0i32,
        digest: vec![1u8, 2u8, 3u8],
    };

    let serialize = serde_json::to_string(&hash_output_rs);
    assert!(serialize.is_ok());
    assert_eq!(serialize.unwrap(), hash_output_json);

    let deserialize = serde_json::from_str::<HashOutput>(hash_output_json);
    assert!(deserialize.is_ok());
    assert_eq!(deserialize.unwrap(), hash_output_rs);
}

/// LogId, a structure with a field using camelCase
#[test]
fn camel_case() {
    let log_id_json = r#"{"keyId":[0]}"#;
    let log_id_rs = LogId { key_id: vec![0] };

    let serialize = serde_json::to_string(&log_id_rs);
    assert!(serialize.is_ok());
    assert_eq!(serialize.unwrap(), log_id_json);

    let deserialize = serde_json::from_str::<LogId>(log_id_json);
    assert!(deserialize.is_ok());
    assert_eq!(deserialize.unwrap(), log_id_rs);
}

/// MessageSignature, nested structure
#[test]
fn nested() {
    let message_signature_json = r#"{
            "messageDigest": {
                "algorithm": 0,
                "digest": [1,2,3]
            },
            "signature": [1]
        }"#;

    let message_signature_rs = MessageSignature {
        message_digest: Some(HashOutput {
            algorithm: 0i32,
            digest: vec![1u8, 2u8, 3u8],
        }),
        signature: vec![1u8],
    };

    let serialize = serde_json::to_string(&message_signature_rs);
    assert!(serialize.is_ok());
    assert_eq!(
        serialize.unwrap(),
        message_signature_json
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
    );

    let deserialize = serde_json::from_str::<MessageSignature>(&message_signature_json);
    assert!(deserialize.is_ok());
    assert_eq!(deserialize.unwrap(), message_signature_rs);
}
