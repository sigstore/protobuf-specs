static FILE_DESCRIPTOR_SET_BYTES: &'static [u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/generated/file_descriptor_set.bin"
));

mod generated;
pub use generated::*;
