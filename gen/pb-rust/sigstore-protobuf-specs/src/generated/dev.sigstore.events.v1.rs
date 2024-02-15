#[derive(
    sigstore_protobuf_specs_derive::Deserialize_proto,
    sigstore_protobuf_specs_derive::Serialize_proto
)]
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(message_name = "dev.sigstore.events.v1.CloudEvent")]
#[prost_reflect(file_descriptor_set_bytes = "crate::FILE_DESCRIPTOR_SET_BYTES")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudEvent {
    /// Required Attributes
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// URI-reference
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub spec_version: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub r#type: ::prost::alloc::string::String,
    /// Optional & Extension Attributes
    #[prost(map = "string, message", tag = "5")]
    pub attributes: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        cloud_event::CloudEventAttributeValue,
    >,
    /// -- CloudEvent Data (Bytes, Text, or Proto)
    #[prost(oneof = "cloud_event::Data", tags = "6, 7, 8")]
    pub data: ::core::option::Option<cloud_event::Data>,
}
/// Nested message and enum types in `CloudEvent`.
pub mod cloud_event {
    #[derive(
        sigstore_protobuf_specs_derive::Deserialize_proto,
        sigstore_protobuf_specs_derive::Serialize_proto
    )]
    #[derive(::prost_reflect::ReflectMessage)]
    #[prost_reflect(
        message_name = "dev.sigstore.events.v1.CloudEvent.CloudEventAttributeValue"
    )]
    #[prost_reflect(file_descriptor_set_bytes = "crate::FILE_DESCRIPTOR_SET_BYTES")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CloudEventAttributeValue {
        #[prost(
            oneof = "cloud_event_attribute_value::Attr",
            tags = "1, 2, 3, 4, 5, 6, 7"
        )]
        pub attr: ::core::option::Option<cloud_event_attribute_value::Attr>,
    }
    /// Nested message and enum types in `CloudEventAttributeValue`.
    pub mod cloud_event_attribute_value {
        #[derive(
            sigstore_protobuf_specs_derive::Deserialize_proto,
            sigstore_protobuf_specs_derive::Serialize_proto
        )]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Attr {
            #[prost(bool, tag = "1")]
            CeBoolean(bool),
            #[prost(int32, tag = "2")]
            CeInteger(i32),
            #[prost(string, tag = "3")]
            CeString(::prost::alloc::string::String),
            #[prost(bytes, tag = "4")]
            CeBytes(::prost::alloc::vec::Vec<u8>),
            #[prost(string, tag = "5")]
            CeUri(::prost::alloc::string::String),
            #[prost(string, tag = "6")]
            CeUriRef(::prost::alloc::string::String),
            #[prost(message, tag = "7")]
            CeTimestamp(::prost_types::Timestamp),
        }
    }
    /// -- CloudEvent Data (Bytes, Text, or Proto)
    #[derive(
        sigstore_protobuf_specs_derive::Deserialize_proto,
        sigstore_protobuf_specs_derive::Serialize_proto
    )]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        #[prost(bytes, tag = "6")]
        BinaryData(::prost::alloc::vec::Vec<u8>),
        #[prost(string, tag = "7")]
        TextData(::prost::alloc::string::String),
        #[prost(message, tag = "8")]
        ProtoData(::prost_types::Any),
    }
}
#[derive(
    sigstore_protobuf_specs_derive::Deserialize_proto,
    sigstore_protobuf_specs_derive::Serialize_proto
)]
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(message_name = "dev.sigstore.events.v1.CloudEventBatch")]
#[prost_reflect(file_descriptor_set_bytes = "crate::FILE_DESCRIPTOR_SET_BYTES")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudEventBatch {
    #[prost(message, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<CloudEvent>,
}
