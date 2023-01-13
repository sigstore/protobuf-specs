# Release management for protocol buffer specifications

This repository primary provides two features:

* Protobuf specifications for messages used within Sigstore.
* Language clients for different ecosystems.

During a release, a few steps have to be synchronized to release the
messages and the language clients.

## Notes on semantic versioning

### Major version change
As expected this indicate a breaking change. Any major update MUST
update the package name of the generated code.

### Minor version change
An update which does not break the functionality of existing (older)
clients. For more information on forward compatible changes in
protobuf see the [Language
Guide](https://developers.google.com/protocol-buffers/docs/proto3#updating).

### Patch version change
Any update which does not change the behaviour. For the protocol buffer
messages this is limited to _only_ capture changes in the comments,
not the messages themselves. For language clients patch versions MAY
be used for bug-fixes.

## Releasing new versions of the messages

Checklist prior to releasing:

1. Gather consensus among the community and maintainers of this
   repository that the messages are ready to be released.
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

When all of the above are set, prepare for release by creating a tag
with the following pattern: `vX.Y.Z` and push to the repository. Bonus
point if the tag is signed :champagne:.

## Releasing new language clients

### Go

No extra step is needed. On every commit a new Go language client is
generated as part of the action
[workflow](https://github.com/sigstore/protobuf-specs/blob/a4750f5ada0d70d66636ab00df7acf694b969750/.github/workflows/generate.yml#L45).

### Java

Prepare a tag with the following pattern `release/java/vX.Y.Z` and
push it. The [workflow](.github/workflows/python-release.yml) will
automatically start.

### Python

Prepare a tag with the following pattern `release/python/vX.Y.Z` and
push it. The [workflow](.github/workflows/java-build-for-release.yml)
will automatically start.
