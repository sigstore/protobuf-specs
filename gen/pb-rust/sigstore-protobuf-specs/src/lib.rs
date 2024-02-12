static FILE_DESCRIPTOR_SET_BYTES: &'static [u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/file_descriptor_set.bin"));

include!(concat!(env!("OUT_DIR"), "/mod.rs"));
