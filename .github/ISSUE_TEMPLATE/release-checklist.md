---
name: Release Checklist
about: All the tasks required to complete a release across languages
title: Release v<fill in>
labels: ''
assignees: ''

---

Full release instructions are at: [RELEASE.md](RELEASE.md)

## Pre Release
- [ ] Check mediatype version of [Bundle](protos/sigstore_bundle.proto), updating for major/minor releases
- [ ] Check mediatype version of [TrustedRoot](protos/sigstore_trustroot.proto), updating for major/minor releases
- [ ] Update [CHANGELOG](CHANGELOG.md)
- [ ] Update [pyproject.toml](gen/pb-python/pyproject.toml) so the `version` matches the targeted release
- [ ] Update [package.json](gen/pb-typescript/package.json) so the `version` matches the targeted release
- [ ] Update [version.rb](gen/pb-ruby/lib/sigstore_protobuf_specs/version.rb) so the `version` matches the targeted release
- [ ] Update [Cargo.toml](gen/pb-rust/Cargo.toml) so the `version` matches the targeted release

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
