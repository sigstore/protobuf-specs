/// HashOutput captures a digest of a 'message' (generic octet sequence)
/// and the corresponding hash algorithm used.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HashOutput {
    #[prost(enumeration = "HashAlgorithm", tag = "1")]
    pub algorithm: i32,
    /// This is the raw octets of the message digest as computed by
    /// the hash algorithm.
    #[prost(bytes = "vec", tag = "2")]
    pub digest: ::prost::alloc::vec::Vec<u8>,
}
/// MessageSignature stores the computed signature over a message.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageSignature {
    /// Message digest can be used to identify the artifact.
    #[prost(message, optional, tag = "1")]
    pub message_digest: ::core::option::Option<HashOutput>,
    /// The raw bytes as returned from the signature algorithm.
    /// The signature algorithm (and so the format of the signature bytes)
    /// are determined by the contents of the 'verification_material',
    /// either a key-pair or a certificate. If using a certificate, the
    /// certificate contains the required information on the signature
    /// algorithm.
    /// When using a key pair, the algorithm MUST be part of the public
    /// key, which MUST be communicated out-of-band.
    #[prost(bytes = "vec", tag = "2")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
/// LogId captures the identity of a transparency log.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogId {
    /// The unique id of the log, represented as the SHA-256 hash
    /// of the log's public key, calculated over the DER encoding
    /// of the key represented as SubjectPublicKeyInfo.
    /// See <https://www.rfc-editor.org/rfc/rfc6962#section-3.2>
    #[prost(bytes = "vec", tag = "1")]
    pub key_id: ::prost::alloc::vec::Vec<u8>,
}
/// This message holds a RFC 3161 timestamp.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rfc3161SignedTimestamp {
    /// Signed timestamp is the DER encoded TimeStampResponse.
    /// See <https://www.rfc-editor.org/rfc/rfc3161.html#section-2.4.2>
    #[prost(bytes = "vec", tag = "1")]
    pub signed_timestamp: ::prost::alloc::vec::Vec<u8>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublicKey {
    /// DER-encoded public key, encoding method is specified by the
    /// key_details attribute.
    #[prost(bytes = "vec", optional, tag = "1")]
    pub raw_bytes: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    /// Key encoding and signature algorithm to use for this key.
    #[prost(enumeration = "PublicKeyDetails", tag = "2")]
    pub key_details: i32,
    /// Optional validity period for this key, *inclusive* of the endpoints.
    #[prost(message, optional, tag = "3")]
    pub valid_for: ::core::option::Option<TimeRange>,
}
/// PublicKeyIdentifier can be used to identify an (out of band) delivered
/// key, to verify a signature.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublicKeyIdentifier {
    /// Optional unauthenticated hint on which key to use.
    /// The format of the hint must be agreed upon out of band by the
    /// signer and the verifiers, and so is not subject to this
    /// specification.
    /// Example use-case is to specify the public key to use, from a
    /// trusted key-ring.
    /// Implementors are RECOMMENDED to derive the value from the public
    /// key as described in RFC 6962.
    /// See: <<https://www.rfc-editor.org/rfc/rfc6962#section-3.2>>
    #[prost(string, tag = "1")]
    pub hint: ::prost::alloc::string::String,
}
/// An ASN.1 OBJECT IDENTIFIER
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectIdentifier {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<i32>,
}
/// An OID and the corresponding (byte) value.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectIdentifierValuePair {
    #[prost(message, optional, tag = "1")]
    pub oid: ::core::option::Option<ObjectIdentifier>,
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DistinguishedName {
    #[prost(string, tag = "1")]
    pub organization: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub common_name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct X509Certificate {
    /// DER-encoded X.509 certificate.
    #[prost(bytes = "vec", tag = "1")]
    pub raw_bytes: ::prost::alloc::vec::Vec<u8>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubjectAlternativeName {
    #[prost(enumeration = "SubjectAlternativeNameType", tag = "1")]
    pub r#type: i32,
    #[prost(oneof = "subject_alternative_name::Identity", tags = "2, 3")]
    pub identity: ::core::option::Option<subject_alternative_name::Identity>,
}
/// Nested message and enum types in `SubjectAlternativeName`.
pub mod subject_alternative_name {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Identity {
        /// A regular expression describing the expected value for
        /// the SAN.
        #[prost(string, tag = "2")]
        Regexp(::prost::alloc::string::String),
        /// The exact value to match against.
        #[prost(string, tag = "3")]
        Value(::prost::alloc::string::String),
    }
}
/// A chain of X.509 certificates.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct X509CertificateChain {
    /// The chain of certificates, with indices 0 to n.
    /// The first certificate in the array must be the leaf
    /// certificate used for signing.
    ///
    /// Signers MUST NOT include their root CA certificates in their embedded
    /// certificate chains, and SHOULD NOT include intermediate CA
    /// certificates that appear in independent roots of trust.
    ///
    /// Verifiers MUST validate the chain carefully to ensure that it chains
    /// up to a root CA certificate that they trust, regardless of whether
    /// the chain includes additional intermediate/root CA certificates.
    /// Verifiers MAY enforce additional constraints, such as requiring that
    /// all intermediate CA certificates appear in an independent root of
    /// trust.
    ///
    /// Verifiers SHOULD handle old or non-complying bundles that have
    /// additional intermediate/root CA certificates.
    #[prost(message, repeated, tag = "1")]
    pub certificates: ::prost::alloc::vec::Vec<X509Certificate>,
}
/// The time range is closed and includes both the start and end times,
/// (i.e., [start, end]).
/// End is optional to be able to capture a period that has started but
/// has no known end.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeRange {
    #[prost(message, optional, tag = "1")]
    pub start: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag = "2")]
    pub end: ::core::option::Option<::pbjson_types::Timestamp>,
}
/// Only a subset of the secure hash standard algorithms are supported.
/// See <<https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf>> for more
/// details.
/// UNSPECIFIED SHOULD not be used, primary reason for inclusion is to force
/// any proto JSON serialization to emit the used hash algorithm, as default
/// option is to *omit* the default value of an enum (which is the first
/// value, represented by '0'.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HashAlgorithm {
    Unspecified = 0,
    Sha2256 = 1,
}
impl HashAlgorithm {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            HashAlgorithm::Unspecified => "HASH_ALGORITHM_UNSPECIFIED",
            HashAlgorithm::Sha2256 => "SHA2_256",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "HASH_ALGORITHM_UNSPECIFIED" => Some(Self::Unspecified),
            "SHA2_256" => Some(Self::Sha2256),
            _ => None,
        }
    }
}
/// Details of a specific public key, capturing the the key encoding method,
/// and signature algorithm.
/// To avoid the possibility of contradicting formats such as PKCS1 with
/// ED25519 the valid permutations are listed as a linear set instead of a
/// cartesian set (i.e one combined variable instead of two, one for encoding
/// and one for the signature algorithm).
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PublicKeyDetails {
    Unspecified = 0,
    /// RSA
    ///
    /// See RFC8017
    Pkcs1RsaPkcs1v5 = 1,
    /// See RFC8017
    Pkcs1RsaPss = 2,
    PkixRsaPkcs1v5 = 3,
    PkixRsaPss = 4,
    /// ECDSA
    ///
    /// See NIST FIPS 186-4
    PkixEcdsaP256Sha256 = 5,
    /// See RFC6979
    PkixEcdsaP256HmacSha256 = 6,
    /// Ed 25519
    ///
    /// See RFC8032
    PkixEd25519 = 7,
}
impl PublicKeyDetails {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PublicKeyDetails::Unspecified => "PUBLIC_KEY_DETAILS_UNSPECIFIED",
            PublicKeyDetails::Pkcs1RsaPkcs1v5 => "PKCS1_RSA_PKCS1V5",
            PublicKeyDetails::Pkcs1RsaPss => "PKCS1_RSA_PSS",
            PublicKeyDetails::PkixRsaPkcs1v5 => "PKIX_RSA_PKCS1V5",
            PublicKeyDetails::PkixRsaPss => "PKIX_RSA_PSS",
            PublicKeyDetails::PkixEcdsaP256Sha256 => "PKIX_ECDSA_P256_SHA_256",
            PublicKeyDetails::PkixEcdsaP256HmacSha256 => "PKIX_ECDSA_P256_HMAC_SHA_256",
            PublicKeyDetails::PkixEd25519 => "PKIX_ED25519",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PUBLIC_KEY_DETAILS_UNSPECIFIED" => Some(Self::Unspecified),
            "PKCS1_RSA_PKCS1V5" => Some(Self::Pkcs1RsaPkcs1v5),
            "PKCS1_RSA_PSS" => Some(Self::Pkcs1RsaPss),
            "PKIX_RSA_PKCS1V5" => Some(Self::PkixRsaPkcs1v5),
            "PKIX_RSA_PSS" => Some(Self::PkixRsaPss),
            "PKIX_ECDSA_P256_SHA_256" => Some(Self::PkixEcdsaP256Sha256),
            "PKIX_ECDSA_P256_HMAC_SHA_256" => Some(Self::PkixEcdsaP256HmacSha256),
            "PKIX_ED25519" => Some(Self::PkixEd25519),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SubjectAlternativeNameType {
    Unspecified = 0,
    Email = 1,
    Uri = 2,
    /// OID 1.3.6.1.4.1.57264.1.7
    /// See <https://github.com/sigstore/fulcio/blob/main/docs/oid-info.md#1361415726417--othername-san>
    /// for more details.
    OtherName = 3,
}
impl SubjectAlternativeNameType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SubjectAlternativeNameType::Unspecified => {
                "SUBJECT_ALTERNATIVE_NAME_TYPE_UNSPECIFIED"
            }
            SubjectAlternativeNameType::Email => "EMAIL",
            SubjectAlternativeNameType::Uri => "URI",
            SubjectAlternativeNameType::OtherName => "OTHER_NAME",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SUBJECT_ALTERNATIVE_NAME_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "EMAIL" => Some(Self::Email),
            "URI" => Some(Self::Uri),
            "OTHER_NAME" => Some(Self::OtherName),
            _ => None,
        }
    }
}
