use crate::dev::sigstore::common::v1::*;
use serde::ser::{Serialize, SerializeStruct, Serializer};

impl Serialize for HashOutput {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("HashOutput", 2)?;
        s.serialize_field("algorithm", &self.algorithm)?;
        s.serialize_field("digest", &self.digest)?;
        s.end()
    }
}

impl Serialize for MessageSignature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("MessageSignature", 2)?;
        s.serialize_field("message_digest", &self.message_digest)?;
        s.serialize_field("signature", &self.signature)?;
        s.end()
    }
}

impl Serialize for LogId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("LogId", 1)?;
        s.serialize_field("keyId", &self.key_id)?;
        s.end()
    }
}

impl Serialize for Rfc3161SignedTimestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Rfc3161SignedTimestamp", 1)?;
        s.serialize_field("signedTimestamp", &self.signed_timestamp)?;
        s.end()
    }
}

impl Serialize for TimeRange {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("TimeRange", 2)?;
        s.serialize_field("start", &self.start)?;
        s.serialize_field("end", &self.end)?;
        s.end()
    }
}

impl Serialize for PublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("PublicKey", 3)?;
        s.serialize_field("rawBytes", &self.raw_bytes)?;
        s.serialize_field("keyDetails", &self.key_details)?;
        s.serialize_field("validFor", &self.valid_for)?;
        s.end()
    }
}
