#[derive(
    sigstore_protobuf_specs_derive::Deserialize_proto,
    sigstore_protobuf_specs_derive::Serialize_proto
)]
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(message_name = "dev.sigstore.signing.v1.FulcioSigningMaterial")]
#[prost_reflect(file_descriptor_set_bytes = "crate::FILE_DESCRIPTOR_SET_BYTES")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FulcioSigningMaterial {
    /// The OIDC identity token to use for retrieving a signing certificate from fulcio.
    #[prost(string, tag = "1")]
    pub identity_token: ::prost::alloc::string::String,
    /// The type of key to use for signing.
    #[prost(
        enumeration = "super::super::common::v1::PublicKeyDetails",
        optional,
        tag = "2"
    )]
    pub key_details: ::core::option::Option<i32>,
}
#[derive(
    sigstore_protobuf_specs_derive::Deserialize_proto,
    sigstore_protobuf_specs_derive::Serialize_proto
)]
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(message_name = "dev.sigstore.signing.v1.SigningOptions")]
#[prost_reflect(file_descriptor_set_bytes = "crate::FILE_DESCRIPTOR_SET_BYTES")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SigningOptions {
    #[prost(oneof = "signing_options::SigningMaterials", tags = "1")]
    pub signing_materials: ::core::option::Option<signing_options::SigningMaterials>,
}
/// Nested message and enum types in `SigningOptions`.
pub mod signing_options {
    #[derive(
        sigstore_protobuf_specs_derive::Deserialize_proto,
        sigstore_protobuf_specs_derive::Serialize_proto
    )]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SigningMaterials {
        /// The OIDC identity token to use for retrieving a signing certificate from fulcio.
        #[prost(message, tag = "1")]
        FulcioSigningMaterial(super::FulcioSigningMaterial),
    }
}
#[derive(
    sigstore_protobuf_specs_derive::Deserialize_proto,
    sigstore_protobuf_specs_derive::Serialize_proto
)]
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(message_name = "dev.sigstore.signing.v1.BundleContentOptions")]
#[prost_reflect(file_descriptor_set_bytes = "crate::FILE_DESCRIPTOR_SET_BYTES")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BundleContentOptions {
    #[prost(oneof = "bundle_content_options::Content", tags = "1, 2")]
    pub content: ::core::option::Option<bundle_content_options::Content>,
}
/// Nested message and enum types in `BundleContentOptions`.
pub mod bundle_content_options {
    #[derive(
        sigstore_protobuf_specs_derive::Deserialize_proto,
        sigstore_protobuf_specs_derive::Serialize_proto
    )]
    #[derive(::prost_reflect::ReflectMessage)]
    #[prost_reflect(
        message_name = "dev.sigstore.signing.v1.BundleContentOptions.MessageSignature"
    )]
    #[prost_reflect(file_descriptor_set_bytes = "crate::FILE_DESCRIPTOR_SET_BYTES")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MessageSignature {
        #[prost(
            enumeration = "super::super::super::common::v1::HashAlgorithm",
            tag = "1"
        )]
        pub hash_algorithm: i32,
    }
    #[derive(
        sigstore_protobuf_specs_derive::Deserialize_proto,
        sigstore_protobuf_specs_derive::Serialize_proto
    )]
    #[derive(::prost_reflect::ReflectMessage)]
    #[prost_reflect(message_name = "dev.sigstore.signing.v1.BundleContentOptions.DSSE")]
    #[prost_reflect(file_descriptor_set_bytes = "crate::FILE_DESCRIPTOR_SET_BYTES")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Dsse {
        #[prost(bytes = "vec", tag = "1")]
        pub payload: ::prost::alloc::vec::Vec<u8>,
        #[prost(string, tag = "2")]
        pub payload_type: ::prost::alloc::string::String,
    }
    #[derive(
        sigstore_protobuf_specs_derive::Deserialize_proto,
        sigstore_protobuf_specs_derive::Serialize_proto
    )]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        #[prost(message, tag = "1")]
        MessageSignature(MessageSignature),
        #[prost(message, tag = "2")]
        DsseEnvelope(Dsse),
    }
}
#[derive(
    sigstore_protobuf_specs_derive::Deserialize_proto,
    sigstore_protobuf_specs_derive::Serialize_proto
)]
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(message_name = "dev.sigstore.signing.v1.TSAOptions")]
#[prost_reflect(file_descriptor_set_bytes = "crate::FILE_DESCRIPTOR_SET_BYTES")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TsaOptions {}
/// Input captures all that is needed to call the signing method.
#[derive(
    sigstore_protobuf_specs_derive::Deserialize_proto,
    sigstore_protobuf_specs_derive::Serialize_proto
)]
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(message_name = "dev.sigstore.signing.v1.SigningInput")]
#[prost_reflect(file_descriptor_set_bytes = "crate::FILE_DESCRIPTOR_SET_BYTES")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SigningInput {
    #[prost(message, optional, tag = "1")]
    pub trusted_root: ::core::option::Option<super::super::trustroot::v1::TrustedRoot>,
    #[prost(message, optional, tag = "2")]
    pub signing_options: ::core::option::Option<SigningOptions>,
    #[prost(message, optional, tag = "3")]
    pub bundle_content_options: ::core::option::Option<BundleContentOptions>,
    #[prost(message, optional, tag = "4")]
    pub tsa_options: ::core::option::Option<TsaOptions>,
    #[prost(message, optional, tag = "5")]
    pub artifact: ::core::option::Option<super::super::verification::v1::Artifact>,
}
#[derive(
    sigstore_protobuf_specs_derive::Deserialize_proto,
    sigstore_protobuf_specs_derive::Serialize_proto
)]
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(message_name = "dev.sigstore.signing.v1.SigningResult")]
#[prost_reflect(file_descriptor_set_bytes = "crate::FILE_DESCRIPTOR_SET_BYTES")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SigningResult {
    #[prost(oneof = "signing_result::Output", tags = "1, 2")]
    pub output: ::core::option::Option<signing_result::Output>,
}
/// Nested message and enum types in `SigningResult`.
pub mod signing_result {
    #[derive(
        sigstore_protobuf_specs_derive::Deserialize_proto,
        sigstore_protobuf_specs_derive::Serialize_proto
    )]
    #[derive(::prost_reflect::ReflectMessage)]
    #[prost_reflect(message_name = "dev.sigstore.signing.v1.SigningResult.Error")]
    #[prost_reflect(file_descriptor_set_bytes = "crate::FILE_DESCRIPTOR_SET_BYTES")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Error {
        #[prost(string, tag = "1")]
        pub message: ::prost::alloc::string::String,
    }
    #[derive(
        sigstore_protobuf_specs_derive::Deserialize_proto,
        sigstore_protobuf_specs_derive::Serialize_proto
    )]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Output {
        #[prost(message, tag = "1")]
        Bundle(super::super::super::bundle::v1::Bundle),
        #[prost(message, tag = "2")]
        Error(Error),
    }
}
