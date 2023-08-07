use sigstore_protobuf_specs::{
    DevSigstoreCommonV1HashOutput as HashOutput,
    DevSigstoreCommonV1MessageSignature as MessageSignature,
};

/// HashOutput, a structure using only primitive types
#[test]
fn primitives() {
    let hash_output = HashOutput {
        algorithm: Some(String::from("SHA2_256")),
        digest: Some(String::from("gC3WD/iDM4AvJYXnMEO9IcNBKF4Zkv5bMXVeHK3q4w4=")),
    };

    assert_eq!(
        hash_output,
        serde_json::to_string(&hash_output)
            .and_then(|s| serde_json::from_str(&s))
            .unwrap()
    )
}

/// MessageSignature, nested structure
#[test]
fn nested() {
    let message_signature= MessageSignature {
        message_digest: Some(HashOutput {
            algorithm: Some(String::from("SHA_256")),
            digest: Some(String::from("gC3WD/iDM4AvJYXnMEO9IcNBKF4Zkv5bMXVeHK3q4w4=")),
        }),
        signature: Some(String::from("MGUCMQCOOJqTY6XWgB64izK2WVP07b0SG9M5WPCwKhfTPwMvtsgUi8KeRGwQkvvLYbKHdqUCMEbOXFG0NMqEQxWVb6rmGnexdADuGf6Jl8qAC8tn67p3QfVoXzMvFA61PzxwVwvb8g=="))
    };

    assert_eq!(
        message_signature,
        serde_json::to_string(&message_signature)
            .and_then(|s| serde_json::from_str(&s))
            .unwrap()
    )
}
