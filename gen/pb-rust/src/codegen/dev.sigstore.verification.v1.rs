/// The identity of a X.509 Certificate signer.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateIdentity {
    /// The X.509v3 issuer extension (OID 1.3.6.1.4.1.57264.1.1)
    #[prost(string, tag = "1")]
    pub issuer: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub san: ::core::option::Option<super::super::common::v1::SubjectAlternativeName>,
    /// An unordered list of OIDs that must be verified.
    /// All OID/values provided in this list MUST exactly match against
    /// the values in the certificate for verification to be successful.
    #[prost(message, repeated, tag = "3")]
    pub oids: ::prost::alloc::vec::Vec<
        super::super::common::v1::ObjectIdentifierValuePair,
    >,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateIdentities {
    #[prost(message, repeated, tag = "1")]
    pub identities: ::prost::alloc::vec::Vec<CertificateIdentity>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublicKeyIdentities {
    #[prost(message, repeated, tag = "1")]
    pub public_keys: ::prost::alloc::vec::Vec<super::super::common::v1::PublicKey>,
}
/// A light-weight set of options/policies for identifying trusted signers,
/// used during verification of a single artifact.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArtifactVerificationOptions {
    /// Optional options for artifact transparency log verification.
    /// If none is provided, the default verification options are:
    /// Threshold: 1
    /// Online verification: false
    /// Disable: false
    #[prost(message, optional, tag = "3")]
    pub tlog_options: ::core::option::Option<artifact_verification_options::TlogOptions>,
    /// Optional options for certificate transparency log verification.
    /// If none is provided, the default verification options are:
    /// Threshold: 1
    /// Detached SCT: false
    /// Disable: false
    #[prost(message, optional, tag = "4")]
    pub ctlog_options: ::core::option::Option<
        artifact_verification_options::CtlogOptions,
    >,
    /// Optional options for certificate signed timestamp verification.
    /// If none is provided, the default verification options are:
    /// Threshold: 1
    /// Disable: false
    #[prost(message, optional, tag = "5")]
    pub tsa_options: ::core::option::Option<
        artifact_verification_options::TimestampAuthorityOptions,
    >,
    /// At least one identity MUST be provided. Providing zero identities
    /// is an error. If at least one provided identity is found as a
    /// signer, the verification is considered successful.
    #[prost(oneof = "artifact_verification_options::Signers", tags = "1, 2")]
    pub signers: ::core::option::Option<artifact_verification_options::Signers>,
}
/// Nested message and enum types in `ArtifactVerificationOptions`.
pub mod artifact_verification_options {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TlogOptions {
        /// Number of transparency logs the entry must appear on.
        #[prost(int32, tag = "1")]
        pub threshold: i32,
        /// Perform an online inclusion proof.
        #[prost(bool, tag = "2")]
        pub perform_online_verification: bool,
        /// Disable verification for transparency logs.
        #[prost(bool, tag = "3")]
        pub disable: bool,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CtlogOptions {
        /// The number of ct transparency logs the certificate must
        /// appear on.
        #[prost(int32, tag = "1")]
        pub threshold: i32,
        /// Expect detached SCTs.
        /// This is not supported right now as we can't capture an
        /// detached SCT in the bundle.
        #[prost(bool, tag = "2")]
        pub detached_sct: bool,
        /// Disable ct transparency log verification
        #[prost(bool, tag = "3")]
        pub disable: bool,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TimestampAuthorityOptions {
        /// The number of signed timestamps that are expected.
        #[prost(int32, tag = "1")]
        pub threshold: i32,
        /// Disable signed timestamp verification.
        #[prost(bool, tag = "2")]
        pub disable: bool,
    }
    /// At least one identity MUST be provided. Providing zero identities
    /// is an error. If at least one provided identity is found as a
    /// signer, the verification is considered successful.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Signers {
        #[prost(message, tag = "1")]
        CertificateIdentities(super::CertificateIdentities),
        /// To simplify verification implementation, the logic for
        /// bundle verification should be implemented as a
        /// higher-order function, where one of argument should be an
        /// interface over the set of trusted public keys, like this:
        /// `Verify(bytes artifact, bytes signature, string key_id)`.
        /// This way the caller is in full control of mapping the
        /// identified (or hinted) key in the bundle to one of the
        /// trusted keys, as this process is inherently application
        /// specific.
        #[prost(message, tag = "2")]
        PublicKeys(super::PublicKeyIdentities),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Artifact {
    #[prost(oneof = "artifact::Data", tags = "1, 2")]
    pub data: ::core::option::Option<artifact::Data>,
}
/// Nested message and enum types in `Artifact`.
pub mod artifact {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        /// Location of the artifact
        #[prost(string, tag = "1")]
        ArtifactUri(::prost::alloc::string::String),
        /// The raw bytes of the artifact
        #[prost(bytes, tag = "2")]
        Artifact(::prost::alloc::vec::Vec<u8>),
    }
}
/// Input captures all that is needed to call the bundle verification method,
/// to verify a single artifact referenced by the bundle.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Input {
    /// The verification materials provided during a bundle verification.
    /// The running process is usually preloaded with a "global"
    /// dev.sisgtore.trustroot.TrustedRoot.v1 instance. Prior to
    /// verifying an artifact (i.e a bundle), and/or based on current
    /// policy, some selection is expected to happen, to filter out the
    /// exact certificate authority to use, which transparency logs are
    /// relevant etc. The result should b ecaptured in the
    /// `artifact_trust_root`.
    #[prost(message, optional, tag = "1")]
    pub artifact_trust_root: ::core::option::Option<
        super::super::trustroot::v1::TrustedRoot,
    >,
    #[prost(message, optional, tag = "2")]
    pub artifact_verification_options: ::core::option::Option<
        ArtifactVerificationOptions,
    >,
    #[prost(message, optional, tag = "3")]
    pub bundle: ::core::option::Option<super::super::bundle::v1::Bundle>,
    /// If the bundle contains a message signature, the artifact must be
    /// provided.
    #[prost(message, optional, tag = "4")]
    pub artifact: ::core::option::Option<Artifact>,
}
