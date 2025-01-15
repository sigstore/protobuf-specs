---
name: Release Checklist
about: All the tasks required to complete a release across languages
title: Release v<fill in>
labels: ''
assignees: ''

---

Full release instructions are at: [RELEASE.md](/sigstore/protobuf-specs/blob/main/RELEASE.md)

## Pre Release
- [ ] Check mediatype version of [Bundle](/sigstore/protobuf-specs/blob/main/protos/sigstore_bundle.proto), updating for major/minor releases
- [ ] Check mediatype version of [TrustedRoot](/sigstore/protobuf-specs/blob/main/protos/sigstore_trustroot.proto), updating for major/minor releases
- [ ] Update [CHANGELOG](/sigstore/protobuf-specs/blob/main/CHANGELOG.md)
- [ ] Update [pyproject.toml](/sigstore/protobuf-specs/blob/main/gen/pb-python/pyproject.toml) so the `version` matches the targeted release
- [ ] Update [package.json](/sigstore/protobuf-specs/blob/main/gen/pb-typescript/package.json) so the `version` matches the targeted release
- [ ] Update [version.rb](/sigstore/protobuf-specs/blob/main/gen/pb-ruby/lib/sigstore_protobuf_specs/version.rb) so the `version` matches the targeted release
- [ ] Update [Cargo.toml](/sigstore/protobuf-specs/blob/main/gen/pb-rust/sigstore-protobuf-specs/Cargo.toml) so the `version` matches the targeted release

## Tag Release
- [ ] `v<version>`
- [ ] `release/java/v<version>`
- [ ] `release/python/v<version>`
- [ ] `release/ruby/v<version>`
- [ ] `release/rust/v<version>`
- [ ] `release/typescript/v<version>`
- [ ] `release/jsonschema/v<version>`

## Publish Release
- [ ] Java to Maven Central

## Verify Releases Published
- [ ] [Java](https://central.sonatype.com/artifact/dev.sigstore/protobuf-specs/)
- [ ] [Python](https://pypi.org/project/sigstore-protobuf-specs/)
- [ ] [Ruby](https://rubygems.org/gems/sigstore_protobuf_specs)
- [ ] [Rust](https://crates.io/crates/sigstore_protobuf_specs)
- [ ] [Typescript](https://www.npmjs.com/package/@sigstore/protobuf-specs)
