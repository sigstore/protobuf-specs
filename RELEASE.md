# Release management for protocol buffer specifications

This repository primary provides two features:

* Protobuf specifications for messages used within Sigstore.
* Language bindings for different ecosystems.

During a release, a few steps have to be synchronized to release the
messages and the language clients.

## Notes on semantic versioning

General information on evolving protocol buffers is described
[here](https://developers.google.com/protocol-buffers/docs/proto3#updating).

### Major version change
As expected this indicate a breaking change. Any major update MUST
update the package name of the generated code.
Examples of breaking changes are (non-complete list):

* Deletion or rename of a field.
* Changing the type of a field.
* Altering the field number (**NEVER DO THIS!**).

### Minor version change
An update which does not break the functionality of existing (older)
clients. For more information on forward compatible changes in
protobuf see the [Language
Guide](https://developers.google.com/protocol-buffers/docs/proto3#updating).

### Patch version change
Any update which does not change the behaviour. For the protocol buffer
messages this is limited to _only_ capture changes in the comments,
not the messages themselves. For language bindings patch versions MAY
be used for bug-fixes.

## Releasing new versions of the messages

Checklist prior to releasing:

1. Gather consensus among the community and maintainers of this
   repository that the messages are ready to be released. Create an
   issue to inform the community. The issue should describe the
   intended release, and any changes it introduces. The issue must be
   open for comments *at least* for a complete week (7 days).
1. Decide the new version of this release. The releases are versioned
   via [semver](https://semver.org/).
1. Two of the messages,
   [Bundle](https://github.com/sigstore/protobuf-specs/blob/main/protos/sigstore_bundle.proto)
   and
   [TrustedRoot](https://github.com/sigstore/protobuf-specs/blob/main/protos/sigstore_trustroot.proto),
   are expected to be persisted and serialized to disk, and exchanged
   via other mechanisms such as the [Sigstore TUF
   root](https://github.com/sigstore/root-signing). Therefore they
   contain a `media_type`. The media types are versioned, and so they
   must be updated appropriately according to semver. Each message
   SHOULD be versioned independently and so MAY differ from the
   targeted release. The media type represents the version of the
   message, not the release. Note that the media type does NOT capture
   the patch version, only major/minor.
1. Update [pyproject.toml](gen/pb-python/pyproject.toml) so the
   `version` matches the targeted release.
1. Update [package.json](gen/pb-typescript/package.json) so the
    `version` matches the targeted release.
1. Update [version.rb](gen/pb-ruby/lib/sigstore_protobuf_specs/version.rb) so the
   `version` matches the targeted release.
1. Update [Cargo.toml](gen/pb-rust/Cargo.toml) so the
   `version` matches the targeted release.
1. Update the [CHANGELOG](https://github.com/sigstore/protobuf-specs/blob/main/CHANGELOG.md).

When all of the above are set, prepare for release by creating a tag
with the following pattern: `vX.Y.Z` and push to the repository. Bonus
point if the tag is signed :champagne:.

## Releasing new language bindings

### Go

Prepare a tag with the pattern `vX.Y.Z` and push it. No workflow is needed.

**WARNING**: Tags should not be updated to a new ref or deleted/recreated after creation.
Go provides a checksum database that persists an immutable mapping between version and ref,
and updating the tag will break clients that have already downloaded the release.

### Java

Prepare a tag with the following pattern `release/java/vX.Y.Z` and
push it. The [workflow](.github/workflows/java-build-for-release.yml) will
automatically start.
After the job is finished, complete the release following [java
release
instructions](https://github.com/sigstore/protobuf-specs/blob/main/java/README.md#releasing).

### Python

Prepare a tag with the following pattern `release/python/vX.Y.Z` and
push it. The [workflow](.github/workflows/python-release.yml)
will automatically start.

### Ruby

Prepare a tag with the following pattern `release/ruby/vX.Y.Z` and
push it. The [workflow](.github/workflows/ruby-release.yml)
will automatically start.

### Rust

Prepare a tag with the following pattern `release/rust/vX.Y.Z` and
push it. The [workflow](.github/workflows/rust-release.yml)
will automatically start.

### TypeScript

Prepare a tag with the following pattern `release/typescript/vX.Y.Z` and
push it. The [workflow](.github/workflows/typescript-release.yml)
will automatically start.

### JSON Schema

Prepare a tag with the pattern `release/jsonschema/vX.Y.Z` and push it.
No workflow is required.
