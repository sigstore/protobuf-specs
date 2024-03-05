/// TransparencyLogInstance describes the immutable parameters from a
/// transparency log.
/// See <https://www.rfc-editor.org/rfc/rfc9162.html#name-log-parameters>
/// for more details.
/// The included parameters are the minimal set required to identify a log,
/// and verify an inclusion proof/promise.
#[derive(
    sigstore_protobuf_specs_derive::Deserialize_proto,
    sigstore_protobuf_specs_derive::Serialize_proto
)]
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(message_name = "dev.sigstore.trustroot.v1.TransparencyLogInstance")]
#[prost_reflect(file_descriptor_set_bytes = "crate::FILE_DESCRIPTOR_SET_BYTES")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransparencyLogInstance {
    /// The base URL at which can be used to URLs for the client.
    #[prost(string, tag = "1")]
    pub base_url: ::prost::alloc::string::String,
    /// The hash algorithm used for the Merkle Tree.
    #[prost(enumeration = "super::super::common::v1::HashAlgorithm", tag = "2")]
    pub hash_algorithm: i32,
    /// The public key used to verify signatures generated by the log.
    /// This attribute contains the signature algorithm used by the log.
    #[prost(message, optional, tag = "3")]
    pub public_key: ::core::option::Option<super::super::common::v1::PublicKey>,
    /// The unique identifier for this transparency log.
    #[prost(message, optional, tag = "4")]
    pub log_id: ::core::option::Option<super::super::common::v1::LogId>,
}
/// CertificateAuthority enlists the information required to identify which
/// CA to use and perform signature verification.
#[derive(
    sigstore_protobuf_specs_derive::Deserialize_proto,
    sigstore_protobuf_specs_derive::Serialize_proto
)]
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(message_name = "dev.sigstore.trustroot.v1.CertificateAuthority")]
#[prost_reflect(file_descriptor_set_bytes = "crate::FILE_DESCRIPTOR_SET_BYTES")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateAuthority {
    /// The root certificate MUST be self-signed, and so the subject and
    /// issuer are the same.
    #[prost(message, optional, tag = "1")]
    pub subject: ::core::option::Option<super::super::common::v1::DistinguishedName>,
    /// The URI identifies the certificate authority.
    ///
    /// It is RECOMMENDED that the URI is the base URL for the certificate
    /// authority, that can be provided to any SDK/client provided
    /// by the certificate authority to interact with the certificate
    /// authority.
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// The certificate chain for this CA. The last certificate in the chain
    /// MUST be the trust anchor. The trust anchor MAY be a self-signed root
    /// CA certificate or MAY be an intermediate CA certificate.
    #[prost(message, optional, tag = "3")]
    pub cert_chain: ::core::option::Option<
        super::super::common::v1::X509CertificateChain,
    >,
    /// The time the *entire* chain was valid. This is at max the
    /// longest interval when *all* certificates in the chain were valid,
    /// but it MAY be shorter. Clients MUST check timestamps against *both*
    /// the `valid_for` time range *and* the entire certificate chain.
    ///
    /// The TimeRange should be considered valid *inclusive* of the
    /// endpoints.
    #[prost(message, optional, tag = "4")]
    pub valid_for: ::core::option::Option<super::super::common::v1::TimeRange>,
}
/// TrustedRoot describes the client's complete set of trusted entities.
/// How the TrustedRoot is populated is not specified, but can be a
/// combination of many sources such as TUF repositories, files on disk etc.
///
/// The TrustedRoot is not meant to be used for any artifact verification, only
/// to capture the complete/global set of trusted verification materials.
/// When verifying an artifact, based on the artifact and policies, a selection
/// of keys/authorities are expected to be extracted and provided to the
/// verification function. This way the set of keys/authorities can be kept to
/// a minimal set by the policy to gain better control over what signatures
/// that are allowed.
///
/// The embedded transparency logs, CT logs, CAs and TSAs MUST include any
/// previously used instance -- otherwise signatures made in the past cannot
/// be verified.
///
/// All the listed instances SHOULD be sorted by the 'valid_for' in ascending
/// order, that is, the oldest instance first. Only the last instance is
/// allowed to have their 'end' timestamp unset. All previous instances MUST
/// have a closed interval of validity. The last instance MAY have a closed
/// interval. Clients MUST accept instances that overlaps in time, if not
/// clients may experience problems during rotations of verification
/// materials.
///
/// To be able to manage planned rotations of either transparency logs or
/// certificate authorities, clienst MUST accept lists of instances where
/// the last instance have a 'valid_for' that belongs to the future.
/// This should not be a problem as clients SHOULD first seek the trust root
/// for a suitable instance before creating a per artifact trust root (that
/// is, a sub-set of the complete trust root) that is used for verification.
#[derive(
    sigstore_protobuf_specs_derive::Deserialize_proto,
    sigstore_protobuf_specs_derive::Serialize_proto
)]
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(message_name = "dev.sigstore.trustroot.v1.TrustedRoot")]
#[prost_reflect(file_descriptor_set_bytes = "crate::FILE_DESCRIPTOR_SET_BYTES")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrustedRoot {
    /// MUST be application/vnd.dev.sigstore.trustedroot+json;version=0.1
    #[prost(string, tag = "1")]
    pub media_type: ::prost::alloc::string::String,
    /// A set of trusted Rekor servers.
    #[prost(message, repeated, tag = "2")]
    pub tlogs: ::prost::alloc::vec::Vec<TransparencyLogInstance>,
    /// A set of trusted certificate authorities (e.g Fulcio), and any
    /// intermediate certificates they provide.
    /// If a CA is issuing multiple intermediate certificate, each
    /// combination shall be represented as separate chain. I.e, a single
    /// root cert may appear in multiple chains but with different
    /// intermediate and/or leaf certificates.
    /// The certificates are intended to be used for verifying artifact
    /// signatures.
    #[prost(message, repeated, tag = "3")]
    pub certificate_authorities: ::prost::alloc::vec::Vec<CertificateAuthority>,
    /// A set of trusted certificate transparency logs.
    #[prost(message, repeated, tag = "4")]
    pub ctlogs: ::prost::alloc::vec::Vec<TransparencyLogInstance>,
    /// A set of trusted timestamping authorities.
    #[prost(message, repeated, tag = "5")]
    pub timestamp_authorities: ::prost::alloc::vec::Vec<CertificateAuthority>,
}
