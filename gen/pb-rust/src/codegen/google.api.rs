/// An indicator of the behavior of a given field (for example, that a field
/// is required in requests, or given as output but ignored as input).
/// This **does not** change the behavior in protocol buffers itself; it only
/// denotes the behavior and may affect how API tooling handles the field.
///
/// Note: This enum **may** receive new values in the future.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FieldBehavior {
    /// Conventional default for enums. Do not use this.
    Unspecified = 0,
    /// Specifically denotes a field as optional.
    /// While all fields in protocol buffers are optional, this may be specified
    /// for emphasis if appropriate.
    Optional = 1,
    /// Denotes a field as required.
    /// This indicates that the field **must** be provided as part of the request,
    /// and failure to do so will cause an error (usually `INVALID_ARGUMENT`).
    Required = 2,
    /// Denotes a field as output only.
    /// This indicates that the field is provided in responses, but including the
    /// field in a request does nothing (the server *must* ignore it and
    /// *must not* throw an error as a result of the field's presence).
    OutputOnly = 3,
    /// Denotes a field as input only.
    /// This indicates that the field is provided in requests, and the
    /// corresponding field is not included in output.
    InputOnly = 4,
    /// Denotes a field as immutable.
    /// This indicates that the field may be set once in a request to create a
    /// resource, but may not be changed thereafter.
    Immutable = 5,
    /// Denotes that a (repeated) field is an unordered list.
    /// This indicates that the service may provide the elements of the list
    /// in any arbitrary order, rather than the order the user originally
    /// provided. Additionally, the list's order may or may not be stable.
    UnorderedList = 6,
}
impl FieldBehavior {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FieldBehavior::Unspecified => "FIELD_BEHAVIOR_UNSPECIFIED",
            FieldBehavior::Optional => "OPTIONAL",
            FieldBehavior::Required => "REQUIRED",
            FieldBehavior::OutputOnly => "OUTPUT_ONLY",
            FieldBehavior::InputOnly => "INPUT_ONLY",
            FieldBehavior::Immutable => "IMMUTABLE",
            FieldBehavior::UnorderedList => "UNORDERED_LIST",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FIELD_BEHAVIOR_UNSPECIFIED" => Some(Self::Unspecified),
            "OPTIONAL" => Some(Self::Optional),
            "REQUIRED" => Some(Self::Required),
            "OUTPUT_ONLY" => Some(Self::OutputOnly),
            "INPUT_ONLY" => Some(Self::InputOnly),
            "IMMUTABLE" => Some(Self::Immutable),
            "UNORDERED_LIST" => Some(Self::UnorderedList),
            _ => None,
        }
    }
}
