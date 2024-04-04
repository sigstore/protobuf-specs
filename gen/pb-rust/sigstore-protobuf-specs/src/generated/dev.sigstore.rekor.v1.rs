/// KindVersion contains the entry's kind and api version.
#[derive(
    sigstore_protobuf_specs_derive::Deserialize_proto,
    sigstore_protobuf_specs_derive::Serialize_proto
)]
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(message_name = "dev.sigstore.rekor.v1.KindVersion")]
#[prost_reflect(file_descriptor_set_bytes = "crate::FILE_DESCRIPTOR_SET_BYTES")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KindVersion {
    /// Kind is the type of entry being stored in the log.
    /// See here for a list: <https://github.com/sigstore/rekor/tree/main/pkg/types>
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// The specific api version of the type.
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
}
/// The checkpoint MUST contain an origin string as a unique log identifier,
/// the tree size, and the root hash. It MAY also be followed by optional data,
/// and clients MUST NOT assume optional data. The checkpoint MUST also contain
/// a signature over the root hash (tree head). The checkpoint MAY contain additional
/// signatures, but the first SHOULD be the signature from the log. Checkpoint contents
/// are concatenated with newlines into a single string.
/// The checkpoint format is described in
/// <https://github.com/transparency-dev/formats/blob/main/log/README.md>
/// and <https://github.com/C2SP/C2SP/blob/main/tlog-checkpoint.md.>
/// An example implementation can be found in <https://github.com/sigstore/rekor/blob/main/pkg/util/signed_note.go>
#[derive(
    sigstore_protobuf_specs_derive::Deserialize_proto,
    sigstore_protobuf_specs_derive::Serialize_proto
)]
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(message_name = "dev.sigstore.rekor.v1.Checkpoint")]
#[prost_reflect(file_descriptor_set_bytes = "crate::FILE_DESCRIPTOR_SET_BYTES")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Checkpoint {
    #[prost(string, tag = "1")]
    pub envelope: ::prost::alloc::string::String,
}
/// InclusionProof is the proof returned from the transparency log. Can
/// be used for offline or online verification against the log.
#[derive(
    sigstore_protobuf_specs_derive::Deserialize_proto,
    sigstore_protobuf_specs_derive::Serialize_proto
)]
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(message_name = "dev.sigstore.rekor.v1.InclusionProof")]
#[prost_reflect(file_descriptor_set_bytes = "crate::FILE_DESCRIPTOR_SET_BYTES")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InclusionProof {
    /// The index of the entry in the tree it was written to.
    #[prost(int64, tag = "1")]
    pub log_index: i64,
    /// The hash digest stored at the root of the merkle tree at the time
    /// the proof was generated.
    #[prost(bytes = "vec", tag = "2")]
    pub root_hash: ::prost::alloc::vec::Vec<u8>,
    /// The size of the merkle tree at the time the proof was generated.
    #[prost(int64, tag = "3")]
    pub tree_size: i64,
    /// A list of hashes required to compute the inclusion proof, sorted
    /// in order from leaf to root.
    /// Note that leaf and root hashes are not included.
    /// The root hash is available separately in this message, and the
    /// leaf hash should be calculated by the client.
    #[prost(bytes = "vec", repeated, tag = "4")]
    pub hashes: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// Signature of the tree head, as of the time of this proof was
    /// generated. See above info on 'Checkpoint' for more details.
    #[prost(message, optional, tag = "5")]
    pub checkpoint: ::core::option::Option<Checkpoint>,
}
/// The inclusion promise is calculated by Rekor. It's calculated as a
/// signature over a canonical JSON serialization of the persisted entry, the
/// log ID, log index and the integration timestamp.
/// See <https://github.com/sigstore/rekor/blob/a6e58f72b6b18cc06cefe61808efd562b9726330/pkg/api/entries.go#L54>
/// The format of the signature depends on the transparency log's public key.
/// If the signature algorithm requires a hash function and/or a signature
/// scheme (e.g. RSA) those has to be retrieved out-of-band from the log's
/// operators, together with the public key.
/// This is used to verify the integration timestamp's value and that the log
/// has promised to include the entry.
#[derive(
    sigstore_protobuf_specs_derive::Deserialize_proto,
    sigstore_protobuf_specs_derive::Serialize_proto
)]
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(message_name = "dev.sigstore.rekor.v1.InclusionPromise")]
#[prost_reflect(file_descriptor_set_bytes = "crate::FILE_DESCRIPTOR_SET_BYTES")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InclusionPromise {
    #[prost(bytes = "vec", tag = "1")]
    pub signed_entry_timestamp: ::prost::alloc::vec::Vec<u8>,
}
/// TransparencyLogEntry captures all the details required from Rekor to
/// reconstruct an entry, given that the payload is provided via other means.
/// This type can easily be created from the existing response from Rekor.
/// Future iterations could rely on Rekor returning the minimal set of
/// attributes (excluding the payload) that are required for verifying the
/// inclusion promise. The inclusion promise (called SignedEntryTimestamp in
/// the response from Rekor) is similar to a Signed Certificate Timestamp
/// as described here <https://www.rfc-editor.org/rfc/rfc6962.html#section-3.2.>
#[derive(
    sigstore_protobuf_specs_derive::Deserialize_proto,
    sigstore_protobuf_specs_derive::Serialize_proto
)]
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(message_name = "dev.sigstore.rekor.v1.TransparencyLogEntry")]
#[prost_reflect(file_descriptor_set_bytes = "crate::FILE_DESCRIPTOR_SET_BYTES")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransparencyLogEntry {
    /// The global index of the entry, used when querying the log by index.
    #[prost(int64, tag = "1")]
    pub log_index: i64,
    /// The unique identifier of the log.
    #[prost(message, optional, tag = "2")]
    pub log_id: ::core::option::Option<super::super::common::v1::LogId>,
    /// The kind (type) and version of the object associated with this
    /// entry. These values are required to construct the entry during
    /// verification.
    #[prost(message, optional, tag = "3")]
    pub kind_version: ::core::option::Option<KindVersion>,
    /// The UNIX timestamp from the log when the entry was persisted.
    #[prost(int64, tag = "4")]
    pub integrated_time: i64,
    /// The inclusion promise/signed entry timestamp from the log.
    /// Required for v0.1 bundles, and MUST be verified.
    /// Optional for >= v0.2 bundles, and SHOULD be verified when present.
    /// Also may be used as a signed timestamp.
    #[prost(message, optional, tag = "5")]
    pub inclusion_promise: ::core::option::Option<InclusionPromise>,
    /// The inclusion proof can be used for offline or online verification
    /// that the entry was appended to the log, and that the log has not been
    /// altered.
    #[prost(message, optional, tag = "6")]
    pub inclusion_proof: ::core::option::Option<InclusionProof>,
    /// Optional. The canonicalized transparency log entry, used to
    /// reconstruct the Signed Entry Timestamp (SET) during verification.
    /// The contents of this field are the same as the `body` field in
    /// a Rekor response, meaning that it does **not** include the "full"
    /// canonicalized form (of log index, ID, etc.) which are
    /// exposed as separate fields. The verifier is responsible for
    /// combining the `canonicalized_body`, `log_index`, `log_id`,
    /// and `integrated_time` into the payload that the SET's signature
    /// is generated over.
    /// This field is intended to be used in cases where the SET cannot be
    /// produced determinisitically (e.g. inconsistent JSON field ordering,
    /// differing whitespace, etc).
    ///
    /// If set, clients MUST verify that the signature referenced in the
    /// `canonicalized_body` matches the signature provided in the
    /// `Bundle.content`.
    /// If not set, clients are responsible for constructing an equivalent
    /// payload from other sources to verify the signature.
    #[prost(bytes = "vec", tag = "7")]
    pub canonicalized_body: ::prost::alloc::vec::Vec<u8>,
}
