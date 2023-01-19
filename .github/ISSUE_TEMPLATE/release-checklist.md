---
name: Release Checklist
about: All the tasks required to complete a release across languages
title: Release v<fill in>
labels: ''
assignees: ''

---

Full release instructions are at: [RELEASE.md](/RELEASE.md)

## Pre Release
- [ ] Check mediatype version of [Bundle](/protos/sigstore_bundle.proto)
- [ ] Check mediatype version of [TrustedRoot](/protos/sigstore_trustroot.proto),
- [ ] Update [pyproject.toml](/gen/pb-python/pyproject.toml) so the `version` matches the targeted release.

## Tag Release
- [ ] `v<version>`
- [ ] `release/java/v<version>`
- [ ] `release/python/v<version>`

## Publish Release
- [ ] Java to Maven Central

## Verify Releases Published
- [ ] [Java](https://central.sonatype.dev/artifact/dev.sigstore/protobuf-specs/0.1.0/versions)
- [ ] [Python](https://pypi.org/project/sigstore-protobuf-specs/)
