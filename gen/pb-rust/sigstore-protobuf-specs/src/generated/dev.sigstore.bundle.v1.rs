/// Various timestamped counter signatures over the artifacts signature.
/// Currently only RFC3161 signatures are provided. More formats may be added
/// in the future.
#[derive(
    sigstore_protobuf_specs_derive::Deserialize_proto,
    sigstore_protobuf_specs_derive::Serialize_proto
)]
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(message_name = "dev.sigstore.bundle.v1.TimestampVerificationData")]
#[prost_reflect(file_descriptor_set_bytes = "crate::FILE_DESCRIPTOR_SET_BYTES")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimestampVerificationData {
    /// A list of RFC3161 signed timestamps provided by the user.
    /// This can be used when the entry has not been stored on a
    /// transparency log, or in conjunction for a stronger trust model.
    /// Clients MUST verify the hashed message in the message imprint
    /// against the signature in the bundle.
    #[prost(message, repeated, tag = "1")]
    pub rfc3161_timestamps: ::prost::alloc::vec::Vec<
        super::super::common::v1::Rfc3161SignedTimestamp,
    >,
}
/// VerificationMaterial captures details on the materials used to verify
/// signatures. This message may be embedded in a DSSE envelope as a signature
/// extension. Specifically, the `ext` field of the extension will expect this
/// message when the signature extension is for Sigstore. This is identified by
/// the `kind` field in the extension, which must be set to
/// application/vnd.dev.sigstore.verificationmaterial;version=0.1 for Sigstore.
/// When used as a DSSE extension, if the `public_key` field is used to indicate
/// the key identifier, it MUST match the `keyid` field of the signature the
/// extension is attached to.
#[derive(
    sigstore_protobuf_specs_derive::Deserialize_proto,
    sigstore_protobuf_specs_derive::Serialize_proto
)]
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(message_name = "dev.sigstore.bundle.v1.VerificationMaterial")]
#[prost_reflect(file_descriptor_set_bytes = "crate::FILE_DESCRIPTOR_SET_BYTES")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerificationMaterial {
    /// An inclusion proof and an optional signed timestamp from the log.
    /// Client verification libraries MAY provide an option to support v0.1
    /// bundles for backwards compatibility, which may contain an inclusion
    /// promise and not an inclusion proof. In this case, the client MUST
    /// validate the promise.
    /// Verifiers SHOULD NOT allow v0.1 bundles if they're used in an
    /// ecosystem which never produced them.
    #[prost(message, repeated, tag = "3")]
    pub tlog_entries: ::prost::alloc::vec::Vec<
        super::super::rekor::v1::TransparencyLogEntry,
    >,
    /// Timestamp may also come from
    /// tlog_entries.inclusion_promise.signed_entry_timestamp.
    #[prost(message, optional, tag = "4")]
    pub timestamp_verification_data: ::core::option::Option<TimestampVerificationData>,
    /// The key material for verification purposes.
    ///
    /// This allows key material to be conveyed in one of three forms:
    ///
    /// 1. An unspecified public key identifier, for retrieving a key
    ///     from an out-of-band mechanism (such as a keyring);
    ///
    /// 2. A sequence of one or more X.509 certificates, of which the first member
    ///     MUST be a leaf certificate conveying the signing key. Subsequent members
    ///     SHOULD be in issuing order, meaning that `n + 1` should be an issuer for `n`.
    ///
    ///     Signers MUST NOT include root CA certificates in bundles, and SHOULD NOT
    ///     include intermediate CA certificates that appear in an independent root of trust
    ///     (such as the Public Good Instance's trusted root).
    ///
    ///     Verifiers MUST validate the chain carefully to ensure that it chains up
    ///     to a CA certificate that they independently trust. Verifiers SHOULD
    ///     handle old or non-complying bundles that have superfluous intermediate and/or
    ///     root CA certificates by either ignoring them or explicitly considering them
    ///     untrusted for the purposes of chain building.
    ///
    /// 3. A single X.509 certificate, which MUST be a leaf certificate conveying
    ///     the signing key.
    ///
    /// When used with the Public Good Instance (PGI) of Sigstore for "keyless" signing
    /// via Fulcio, form (1) MUST NOT be used, regardless of bundle version. Form (1)
    /// MAY be used with the PGI for self-managed keys.
    ///
    /// When used in a `0.1` or `0.2` bundle with the PGI and "keyless" signing,
    /// form (2) MUST be used.
    ///
    /// When used in a `0.3` bundle with the PGI and "keyless" signing,
    /// form (3) MUST be used.
    #[prost(oneof = "verification_material::Content", tags = "1, 2, 5")]
    pub content: ::core::option::Option<verification_material::Content>,
}
/// Nested message and enum types in `VerificationMaterial`.
pub mod verification_material {
    /// The key material for verification purposes.
    ///
    /// This allows key material to be conveyed in one of three forms:
    ///
    /// 1. An unspecified public key identifier, for retrieving a key
    ///     from an out-of-band mechanism (such as a keyring);
    ///
    /// 2. A sequence of one or more X.509 certificates, of which the first member
    ///     MUST be a leaf certificate conveying the signing key. Subsequent members
    ///     SHOULD be in issuing order, meaning that `n + 1` should be an issuer for `n`.
    ///
    ///     Signers MUST NOT include root CA certificates in bundles, and SHOULD NOT
    ///     include intermediate CA certificates that appear in an independent root of trust
    ///     (such as the Public Good Instance's trusted root).
    ///
    ///     Verifiers MUST validate the chain carefully to ensure that it chains up
    ///     to a CA certificate that they independently trust. Verifiers SHOULD
    ///     handle old or non-complying bundles that have superfluous intermediate and/or
    ///     root CA certificates by either ignoring them or explicitly considering them
    ///     untrusted for the purposes of chain building.
    ///
    /// 3. A single X.509 certificate, which MUST be a leaf certificate conveying
    ///     the signing key.
    ///
    /// When used with the Public Good Instance (PGI) of Sigstore for "keyless" signing
    /// via Fulcio, form (1) MUST NOT be used, regardless of bundle version. Form (1)
    /// MAY be used with the PGI for self-managed keys.
    ///
    /// When used in a `0.1` or `0.2` bundle with the PGI and "keyless" signing,
    /// form (2) MUST be used.
    ///
    /// When used in a `0.3` bundle with the PGI and "keyless" signing,
    /// form (3) MUST be used.
    #[derive(
        sigstore_protobuf_specs_derive::Deserialize_proto,
        sigstore_protobuf_specs_derive::Serialize_proto
    )]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        #[prost(message, tag = "1")]
        PublicKey(super::super::super::common::v1::PublicKeyIdentifier),
        #[prost(message, tag = "2")]
        X509CertificateChain(super::super::super::common::v1::X509CertificateChain),
        #[prost(message, tag = "5")]
        Certificate(super::super::super::common::v1::X509Certificate),
    }
}
#[derive(
    sigstore_protobuf_specs_derive::Deserialize_proto,
    sigstore_protobuf_specs_derive::Serialize_proto
)]
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(message_name = "dev.sigstore.bundle.v1.Bundle")]
#[prost_reflect(file_descriptor_set_bytes = "crate::FILE_DESCRIPTOR_SET_BYTES")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bundle {
    /// MUST be application/vnd.dev.sigstore.bundle+json;version=0.1
    /// or application/vnd.dev.sigstore.bundle+json;version=0.2
    /// or application/vnd.dev.sigstore.bundle+json;version=0.3
    /// when encoded as JSON.
    #[prost(string, tag = "1")]
    pub media_type: ::prost::alloc::string::String,
    /// When a signer is identified by a X.509 certificate, a verifier MUST
    /// verify that the signature was computed at the time the certificate
    /// was valid as described in the Sigstore client spec: "Verification
    /// using a Bundle".
    /// <<https://docs.google.com/document/d/1kbhK2qyPPk8SLavHzYSDM8-Ueul9_oxIMVFuWMWKz0E/edit#heading=h.x8bduppe89ln>>
    /// If the verification material contains a public key identifier
    /// (key hint) and the `content` is a DSSE envelope, the key hints
    /// MUST be exactly the same in the verification material and in the
    /// DSSE envelope.
    #[prost(message, optional, tag = "2")]
    pub verification_material: ::core::option::Option<VerificationMaterial>,
    #[prost(oneof = "bundle::Content", tags = "3, 4")]
    pub content: ::core::option::Option<bundle::Content>,
}
/// Nested message and enum types in `Bundle`.
pub mod bundle {
    #[derive(
        sigstore_protobuf_specs_derive::Deserialize_proto,
        sigstore_protobuf_specs_derive::Serialize_proto
    )]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        #[prost(message, tag = "3")]
        MessageSignature(super::super::super::common::v1::MessageSignature),
        /// A DSSE envelope can contain arbitrary payloads.
        /// Verifiers must verify that the payload type is a
        /// supported and expected type. This is part of the DSSE
        /// protocol which is defined here:
        /// <<https://github.com/secure-systems-lab/dsse/blob/master/protocol.md>>
        #[prost(message, tag = "4")]
        DsseEnvelope(super::super::super::super::super::io::intoto::Envelope),
    }
}
