/// HashOutput captures a digest of a 'message' (generic octet sequence)
/// and the corresponding hash algorithm used.
#[derive(
    sigstore_protobuf_specs_derive::Deserialize_proto,
    sigstore_protobuf_specs_derive::Serialize_proto
)]
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(message_name = "dev.sigstore.common.v1.HashOutput")]
#[prost_reflect(file_descriptor_set_bytes = "crate::FILE_DESCRIPTOR_SET_BYTES")]
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
#[derive(
    sigstore_protobuf_specs_derive::Deserialize_proto,
    sigstore_protobuf_specs_derive::Serialize_proto
)]
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(message_name = "dev.sigstore.common.v1.MessageSignature")]
#[prost_reflect(file_descriptor_set_bytes = "crate::FILE_DESCRIPTOR_SET_BYTES")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageSignature {
    /// Message digest can be used to identify the artifact.
    /// Clients MUST NOT attempt to use this digest to verify the associated
    /// signature; it is intended solely for identification.
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
#[derive(
    sigstore_protobuf_specs_derive::Deserialize_proto,
    sigstore_protobuf_specs_derive::Serialize_proto
)]
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(message_name = "dev.sigstore.common.v1.LogId")]
#[prost_reflect(file_descriptor_set_bytes = "crate::FILE_DESCRIPTOR_SET_BYTES")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogId {
    /// The unique identity of the log, represented by its public key.
    #[prost(bytes = "vec", tag = "1")]
    pub key_id: ::prost::alloc::vec::Vec<u8>,
}
/// This message holds a RFC 3161 timestamp.
#[derive(
    sigstore_protobuf_specs_derive::Deserialize_proto,
    sigstore_protobuf_specs_derive::Serialize_proto
)]
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(message_name = "dev.sigstore.common.v1.RFC3161SignedTimestamp")]
#[prost_reflect(file_descriptor_set_bytes = "crate::FILE_DESCRIPTOR_SET_BYTES")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rfc3161SignedTimestamp {
    /// Signed timestamp is the DER encoded TimeStampResponse.
    /// See <https://www.rfc-editor.org/rfc/rfc3161.html#section-2.4.2>
    #[prost(bytes = "vec", tag = "1")]
    pub signed_timestamp: ::prost::alloc::vec::Vec<u8>,
}
#[derive(
    sigstore_protobuf_specs_derive::Deserialize_proto,
    sigstore_protobuf_specs_derive::Serialize_proto
)]
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(message_name = "dev.sigstore.common.v1.PublicKey")]
#[prost_reflect(file_descriptor_set_bytes = "crate::FILE_DESCRIPTOR_SET_BYTES")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublicKey {
    /// DER-encoded public key, encoding method is specified by the
    /// key_details attribute.
    #[prost(bytes = "vec", tag = "1")]
    pub raw_bytes: ::prost::alloc::vec::Vec<u8>,
    /// Key encoding and signature algorithm to use for this key.
    #[prost(enumeration = "PublicKeyDetails", tag = "2")]
    pub key_details: i32,
    /// Optional validity period for this key, *inclusive* of the endpoints.
    #[prost(message, optional, tag = "3")]
    pub valid_for: ::core::option::Option<TimeRange>,
}
/// PublicKeyIdentifier can be used to identify an (out of band) delivered
/// key, to verify a signature.
#[derive(
    sigstore_protobuf_specs_derive::Deserialize_proto,
    sigstore_protobuf_specs_derive::Serialize_proto
)]
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(message_name = "dev.sigstore.common.v1.PublicKeyIdentifier")]
#[prost_reflect(file_descriptor_set_bytes = "crate::FILE_DESCRIPTOR_SET_BYTES")]
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
#[derive(
    sigstore_protobuf_specs_derive::Deserialize_proto,
    sigstore_protobuf_specs_derive::Serialize_proto
)]
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(message_name = "dev.sigstore.common.v1.ObjectIdentifier")]
#[prost_reflect(file_descriptor_set_bytes = "crate::FILE_DESCRIPTOR_SET_BYTES")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectIdentifier {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<i32>,
}
/// An OID and the corresponding (byte) value.
#[derive(
    sigstore_protobuf_specs_derive::Deserialize_proto,
    sigstore_protobuf_specs_derive::Serialize_proto
)]
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(message_name = "dev.sigstore.common.v1.ObjectIdentifierValuePair")]
#[prost_reflect(file_descriptor_set_bytes = "crate::FILE_DESCRIPTOR_SET_BYTES")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectIdentifierValuePair {
    #[prost(message, optional, tag = "1")]
    pub oid: ::core::option::Option<ObjectIdentifier>,
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[derive(
    sigstore_protobuf_specs_derive::Deserialize_proto,
    sigstore_protobuf_specs_derive::Serialize_proto
)]
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(message_name = "dev.sigstore.common.v1.DistinguishedName")]
#[prost_reflect(file_descriptor_set_bytes = "crate::FILE_DESCRIPTOR_SET_BYTES")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DistinguishedName {
    #[prost(string, tag = "1")]
    pub organization: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub common_name: ::prost::alloc::string::String,
}
#[derive(
    sigstore_protobuf_specs_derive::Deserialize_proto,
    sigstore_protobuf_specs_derive::Serialize_proto
)]
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(message_name = "dev.sigstore.common.v1.X509Certificate")]
#[prost_reflect(file_descriptor_set_bytes = "crate::FILE_DESCRIPTOR_SET_BYTES")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct X509Certificate {
    /// DER-encoded X.509 certificate.
    #[prost(bytes = "vec", tag = "1")]
    pub raw_bytes: ::prost::alloc::vec::Vec<u8>,
}
#[derive(
    sigstore_protobuf_specs_derive::Deserialize_proto,
    sigstore_protobuf_specs_derive::Serialize_proto
)]
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(message_name = "dev.sigstore.common.v1.SubjectAlternativeName")]
#[prost_reflect(file_descriptor_set_bytes = "crate::FILE_DESCRIPTOR_SET_BYTES")]
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
    #[derive(
        sigstore_protobuf_specs_derive::Deserialize_proto,
        sigstore_protobuf_specs_derive::Serialize_proto
    )]
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
/// A collection of X.509 certificates.
///
/// This "chain" can be used in multiple contexts, such as providing a root CA
/// certificate within a TUF root of trust or multiple untrusted certificates for
/// the purpose of chain building.
#[derive(
    sigstore_protobuf_specs_derive::Deserialize_proto,
    sigstore_protobuf_specs_derive::Serialize_proto
)]
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(message_name = "dev.sigstore.common.v1.X509CertificateChain")]
#[prost_reflect(file_descriptor_set_bytes = "crate::FILE_DESCRIPTOR_SET_BYTES")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct X509CertificateChain {
    /// One or more DER-encoded certificates.
    ///
    /// In some contexts (such as `VerificationMaterial.x509_certificate_chain`), this sequence
    /// has an imposed order. Unless explicitly specified, there is otherwise no
    /// guaranteed order.
    #[prost(message, repeated, tag = "1")]
    pub certificates: ::prost::alloc::vec::Vec<X509Certificate>,
}
/// The time range is closed and includes both the start and end times,
/// (i.e., \[start, end\]).
/// End is optional to be able to capture a period that has started but
/// has no known end.
#[derive(
    sigstore_protobuf_specs_derive::Deserialize_proto,
    sigstore_protobuf_specs_derive::Serialize_proto
)]
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(message_name = "dev.sigstore.common.v1.TimeRange")]
#[prost_reflect(file_descriptor_set_bytes = "crate::FILE_DESCRIPTOR_SET_BYTES")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeRange {
    #[prost(message, optional, tag = "1")]
    pub start: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "2")]
    pub end: ::core::option::Option<::prost_types::Timestamp>,
}
/// Only a subset of the secure hash standard algorithms are supported.
/// See <<https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf>> for more
/// details.
/// UNSPECIFIED SHOULD not be used, primary reason for inclusion is to force
/// any proto JSON serialization to emit the used hash algorithm, as default
/// option is to *omit* the default value of an enum (which is the first
/// value, represented by '0'.
#[derive(
    sigstore_protobuf_specs_derive::Deserialize_proto,
    sigstore_protobuf_specs_derive::Serialize_proto
)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HashAlgorithm {
    Unspecified = 0,
    Sha2256 = 1,
    Sha2384 = 2,
    Sha2512 = 3,
    Sha3256 = 4,
    Sha3384 = 5,
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
            HashAlgorithm::Sha2384 => "SHA2_384",
            HashAlgorithm::Sha2512 => "SHA2_512",
            HashAlgorithm::Sha3256 => "SHA3_256",
            HashAlgorithm::Sha3384 => "SHA3_384",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "HASH_ALGORITHM_UNSPECIFIED" => Some(Self::Unspecified),
            "SHA2_256" => Some(Self::Sha2256),
            "SHA2_384" => Some(Self::Sha2384),
            "SHA2_512" => Some(Self::Sha2512),
            "SHA3_256" => Some(Self::Sha3256),
            "SHA3_384" => Some(Self::Sha3384),
            _ => None,
        }
    }
}
/// Details of a specific public key, capturing the the key encoding method,
/// and signature algorithm.
///
/// PublicKeyDetails captures the public key/hash algorithm combinations
/// recommended in the Sigstore ecosystem.
///
/// This is modelled as a linear set as we want to provide a small number of
/// opinionated options instead of allowing every possible permutation.
///
/// Any changes to this enum MUST be reflected in the algorithm registry.
/// See: docs/algorithm-registry.md
///
/// To avoid the possibility of contradicting formats such as PKCS1 with
/// ED25519 the valid permutations are listed as a linear set instead of a
/// cartesian set (i.e one combined variable instead of two, one for encoding
/// and one for the signature algorithm).
#[derive(
    sigstore_protobuf_specs_derive::Deserialize_proto,
    sigstore_protobuf_specs_derive::Serialize_proto
)]
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
    /// RSA public key in PKIX format, PKCS#1v1.5 signature
    PkixRsaPkcs1v152048Sha256 = 9,
    PkixRsaPkcs1v153072Sha256 = 10,
    PkixRsaPkcs1v154096Sha256 = 11,
    /// RSA public key in PKIX format, RSASSA-PSS signature
    ///
    /// See RFC4055
    PkixRsaPss2048Sha256 = 16,
    PkixRsaPss3072Sha256 = 17,
    PkixRsaPss4096Sha256 = 18,
    /// ECDSA
    ///
    /// See RFC6979
    PkixEcdsaP256HmacSha256 = 6,
    /// See NIST FIPS 186-4
    PkixEcdsaP256Sha256 = 5,
    PkixEcdsaP384Sha384 = 12,
    PkixEcdsaP521Sha512 = 13,
    /// Ed 25519
    ///
    /// See RFC8032
    PkixEd25519 = 7,
    PkixEd25519Ph = 8,
    /// LMS and LM-OTS
    ///
    /// These keys and signatures may be used by private Sigstore
    /// deployments, but are not currently supported by the public
    /// good instance.
    ///
    /// USER WARNING: LMS and LM-OTS are both stateful signature schemes.
    /// Using them correctly requires discretion and careful consideration
    /// to ensure that individual secret keys are not used more than once.
    /// In addition, LM-OTS is a single-use scheme, meaning that it
    /// MUST NOT be used for more than one signature per LM-OTS key.
    /// If you cannot maintain these invariants, you MUST NOT use these
    /// schemes.
    LmsSha256 = 14,
    LmotsSha256 = 15,
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
            PublicKeyDetails::PkixRsaPkcs1v152048Sha256 => {
                "PKIX_RSA_PKCS1V15_2048_SHA256"
            }
            PublicKeyDetails::PkixRsaPkcs1v153072Sha256 => {
                "PKIX_RSA_PKCS1V15_3072_SHA256"
            }
            PublicKeyDetails::PkixRsaPkcs1v154096Sha256 => {
                "PKIX_RSA_PKCS1V15_4096_SHA256"
            }
            PublicKeyDetails::PkixRsaPss2048Sha256 => "PKIX_RSA_PSS_2048_SHA256",
            PublicKeyDetails::PkixRsaPss3072Sha256 => "PKIX_RSA_PSS_3072_SHA256",
            PublicKeyDetails::PkixRsaPss4096Sha256 => "PKIX_RSA_PSS_4096_SHA256",
            PublicKeyDetails::PkixEcdsaP256HmacSha256 => "PKIX_ECDSA_P256_HMAC_SHA_256",
            PublicKeyDetails::PkixEcdsaP256Sha256 => "PKIX_ECDSA_P256_SHA_256",
            PublicKeyDetails::PkixEcdsaP384Sha384 => "PKIX_ECDSA_P384_SHA_384",
            PublicKeyDetails::PkixEcdsaP521Sha512 => "PKIX_ECDSA_P521_SHA_512",
            PublicKeyDetails::PkixEd25519 => "PKIX_ED25519",
            PublicKeyDetails::PkixEd25519Ph => "PKIX_ED25519_PH",
            PublicKeyDetails::LmsSha256 => "LMS_SHA256",
            PublicKeyDetails::LmotsSha256 => "LMOTS_SHA256",
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
            "PKIX_RSA_PKCS1V15_2048_SHA256" => Some(Self::PkixRsaPkcs1v152048Sha256),
            "PKIX_RSA_PKCS1V15_3072_SHA256" => Some(Self::PkixRsaPkcs1v153072Sha256),
            "PKIX_RSA_PKCS1V15_4096_SHA256" => Some(Self::PkixRsaPkcs1v154096Sha256),
            "PKIX_RSA_PSS_2048_SHA256" => Some(Self::PkixRsaPss2048Sha256),
            "PKIX_RSA_PSS_3072_SHA256" => Some(Self::PkixRsaPss3072Sha256),
            "PKIX_RSA_PSS_4096_SHA256" => Some(Self::PkixRsaPss4096Sha256),
            "PKIX_ECDSA_P256_HMAC_SHA_256" => Some(Self::PkixEcdsaP256HmacSha256),
            "PKIX_ECDSA_P256_SHA_256" => Some(Self::PkixEcdsaP256Sha256),
            "PKIX_ECDSA_P384_SHA_384" => Some(Self::PkixEcdsaP384Sha384),
            "PKIX_ECDSA_P521_SHA_512" => Some(Self::PkixEcdsaP521Sha512),
            "PKIX_ED25519" => Some(Self::PkixEd25519),
            "PKIX_ED25519_PH" => Some(Self::PkixEd25519Ph),
            "LMS_SHA256" => Some(Self::LmsSha256),
            "LMOTS_SHA256" => Some(Self::LmotsSha256),
            _ => None,
        }
    }
}
#[derive(
    sigstore_protobuf_specs_derive::Deserialize_proto,
    sigstore_protobuf_specs_derive::Serialize_proto
)]
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
