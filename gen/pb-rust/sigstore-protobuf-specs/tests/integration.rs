use std::io;

use sigstore_protobuf_specs::dev::sigstore::bundle::v1::Bundle;

macro_rules! include_asset {
    ($path:literal) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/", $path))
    };
}

struct SpaceSeparatorFormatter;

impl serde_json::ser::Formatter for SpaceSeparatorFormatter {
    fn begin_array_value<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        if first {
            Ok(())
        } else {
            writer.write_all(b", ")
        }
    }

    fn begin_object_value<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b": ")
    }

    fn begin_object_key<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        if first {
            Ok(())
        } else {
            writer.write_all(b", ")
        }
    }

    fn write_string_fragment<W>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        // Replace em-dashes with a unicode escape. serde_json unescapes it.
        writer.write_all(fragment.replace("\u{2014}", "\\u2014").as_bytes())
    }
}

/// Test re-serializing a known-good bundle from sigstore-python.
#[test]
fn bundle_roundtrip() {
    // Deserialize bundle, trimming trailing whitespace.
    let input = include_asset!("a.txt.sigstore").trim_end();
    let bundle: Bundle = serde_json::from_str(input).expect("failed to deserialize Bundle!");

    // Re-serialize bundle with our python-like formatter.
    let formatter = SpaceSeparatorFormatter {};
    let mut result = Vec::new();
    let mut ser = serde_json::Serializer::with_formatter(&mut result, formatter);
    serde::Serialize::serialize(&bundle, &mut ser).expect("failed to re-serialize Bundle!");

    // Notwithstanding the workarounds above, our serialized bundle should be
    // byte-for-byte identical to the input bundle.
    let result = std::str::from_utf8(&result).unwrap();
    assert_eq!(
        input,
        &result[..],
        "re-serialized Bundle does not match original!"
    );
}
