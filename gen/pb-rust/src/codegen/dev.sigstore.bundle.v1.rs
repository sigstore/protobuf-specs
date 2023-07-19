/// Various timestamped counter signatures over the artifacts signature.
/// Currently only RFC3161 signatures are provided. More formats may be added
/// in the future.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
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
/// signatures.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerificationMaterial {
    /// This is the inclusion proof, where the timestamp is coming from
    /// the transparency log.
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
    /// Timestamp verification data, over the artifact's signature.
    #[prost(message, optional, tag = "4")]
    pub timestamp_verification_data: ::core::option::Option<TimestampVerificationData>,
    #[prost(oneof = "verification_material::Content", tags = "1, 2")]
    pub content: ::core::option::Option<verification_material::Content>,
}
/// Nested message and enum types in `VerificationMaterial`.
pub mod verification_material {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        #[prost(message, tag = "1")]
        PublicKey(super::super::super::common::v1::PublicKeyIdentifier),
        #[prost(message, tag = "2")]
        X509CertificateChain(super::super::super::common::v1::X509CertificateChain),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bundle {
    /// MUST be application/vnd.dev.sigstore.bundle+json;version=0.1
    /// or application/vnd.dev.sigstore.bundle+json;version=0.2
    /// when encoded as JSON.
    #[prost(string, tag = "1")]
    pub media_type: ::prost::alloc::string::String,
    /// When a signer is identified by a X.509 certificate, a verifier MUST
    /// verify that the signature was computed at the time the certificate
    /// was valid as described in the Sigstore client spec: "Verification
    /// using a Bundle".
    /// <<https://docs.google.com/document/d/1kbhK2qyPPk8SLavHzYSDM8-Ueul9_oxIMVFuWMWKz0E/edit#heading=h.x8bduppe89ln>>
    #[prost(message, optional, tag = "2")]
    pub verification_material: ::core::option::Option<VerificationMaterial>,
    #[prost(oneof = "bundle::Content", tags = "3, 4")]
    pub content: ::core::option::Option<bundle::Content>,
}
/// Nested message and enum types in `Bundle`.
pub mod bundle {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
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
