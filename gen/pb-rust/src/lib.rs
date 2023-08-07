/// NOTE(jleightcap): a method to include all JSON schemas is not immediately obvious to me:
///
/// - `schemafy!` is a direct 1:1 compilation to Rust structures of each definition in "defintions",
/// - each schema is 'standalone': including definitions of all it's dependencies,
/// as a result, the Rust generated structures have name collisions if included in the same module scope.
///
/// prefixing works,
///
/// ```ignore
/// mod CertificateIdentity {
///   schemafy::schemafy!("schemas/CertificateIdentity.shema.json")
/// }
/// mod CertificateIdentities {
///   schemafy::schemafy!("schemas/CertificateIdentities.schema.json")
/// }
/// ```
/// but is clunky to use and manual to generate.
///
/// a more general approach is to 'flatten' each JSON Schema,
/// unioning all of the "defintions" fields.
///
/// Since standardized bundles is the singular motiviation for these Rust bindings,
/// for now we're punting this issue.
use serde::{Deserialize, Serialize};
schemafy::schemafy!("schemas/Bundle.schema.json");
