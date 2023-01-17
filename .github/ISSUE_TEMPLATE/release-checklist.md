---
name: Release Checklist
about: All the tasks required to complete a release across languages
title: Release v<fill in>
labels: ''
assignees: ''

---

Full release instructions are at: [RELEASE.md](https://github.com/sigstore/protobuf-specs/RELEASE.md)

## Pre Release
- [ ] Check mediatype version of [Bundle](https://github.com/sigstore/protobuf-specs/blob/main/protos/sigstore_bundle.proto)
- [ ] Check mediatype version of [TrustedRoot](https://github.com/sigstore/protobuf-specs/blob/main/protos/sigstore_trustroot.proto),
- [ ] Update [pyproject.toml](gen/pb-python/pyproject.toml) so the `version` matches the targeted release.

## Releases Complete
- [ ] Java
- [ ] Go
- [ ] Python
- [ ] Node.js
