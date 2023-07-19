/// An authenticated message of arbitrary type.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Envelope {
    /// Message to be signed. (In JSON, this is encoded as base64.)
    /// REQUIRED.
    #[prost(bytes = "vec", tag = "1")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
    /// String unambiguously identifying how to interpret payload.
    /// REQUIRED.
    #[prost(string, tag = "2")]
    pub payload_type: ::prost::alloc::string::String,
    /// Signature over:
    ///      PAE(type, payload)
    /// Where PAE is defined as:
    /// PAE(type, payload) = "DSSEv1" + SP + LEN(type) + SP + type + SP + LEN(payload) + SP + payload
    /// +               = concatenation
    /// SP              = ASCII space \[0x20\]
    /// "DSSEv1"        = ASCII [0x44, 0x53, 0x53, 0x45, 0x76, 0x31]
    /// LEN(s)          = ASCII decimal encoding of the byte length of s, with no leading zeros
    /// REQUIRED (length >= 1).
    #[prost(message, repeated, tag = "3")]
    pub signatures: ::prost::alloc::vec::Vec<Signature>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Signature {
    /// Signature itself. (In JSON, this is encoded as base64.)
    /// REQUIRED.
    #[prost(bytes = "vec", tag = "1")]
    pub sig: ::prost::alloc::vec::Vec<u8>,
    /// *Unauthenticated* hint identifying which public key was used.
    /// OPTIONAL.
    #[prost(string, tag = "2")]
    pub keyid: ::prost::alloc::string::String,
}
