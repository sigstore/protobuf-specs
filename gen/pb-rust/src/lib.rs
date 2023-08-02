use schemafy_core;
use serde;
use serde_json;

use serde::{Deserialize, Serialize};

pub mod sigstore {
    pub mod verification {
        pub mod v1 {
            use super::super::super::*;
            schemafy::schemafy!("schema/Artifact.schema.json");
        }
    }
}
