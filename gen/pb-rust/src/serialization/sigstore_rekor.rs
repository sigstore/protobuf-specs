use crate::dev::sigstore::rekor::v1::*;
use serde::ser::{Serialize, SerializeStruct, Serializer};

/*
impl Serialize for TransparencyLogEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("TransparencyLogEntry", 7)?;
        s.serialize_field("logIndex", &self.log_index.to_string());
        s.serialize_field("logId", &self.log_id);
        s.end();
    }
}
*/

impl Serialize for KindVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("KindVersion", 2)?;
        s.serialize_field("kind", &self.kind)?;
        s.serialize_field("version", &self.version)?;
        s.end()
    }
}
