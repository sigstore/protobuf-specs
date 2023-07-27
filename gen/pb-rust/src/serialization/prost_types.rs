use prost_types::*;
use serde::ser::{Serialize, SerializeStruct, Serializer};

impl Serialize for Timestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Timestamp", 2)?;
        s.serialize_field("seconds", &self.seconds.to_string())?;
        s.serialize_field("nanos", &self.nanos.to_string())?;
        s.end()
    }
}
