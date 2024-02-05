# Changelog

All notable changes to `protobuf-specs` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

All versions prior to 0.2.0 are untracked.

## [Unreleased]

## 0.3.0

* Options for more generic observer time ([#179](https://github.com/sigstore/protobuf-specs/pull/179))
* **BREAKING**: `VerificationMaterials.contents` now has an additional `certificate` variant,
  which is preferred in `0.3` bundles with the Sigstore PGI ([#191](https://github.com/sigstore/protobuf-specs/pull/191))
* Added algorithm registry documentation and update `PublicKeyDetails` message
  ([#194](https://github.com/sigstore/protobuf-specs/pull/194), [#212](https://github.com/sigstore/protobuf-specs/pull/212))
    * Deterministic ECDSA is **deprecated**
    * NIST-P384 and NIST-P521 curves **added**
    * Existing (and underspecified) RSA key types are
      **deprecated**. New RSA keytypes are defined that specifies size
      of public modulus and hash algorithm. RSA now only supports
      [PKCS#1](https://datatracker.ietf.org/doc/html/rfc8017#section-8.2)
      signature scheme, and PKIX
      ([SubjectPublicKeyInfo](https://datatracker.ietf.org/doc/html/rfc5280#section-4.1))
      encoding.
    * Experimental support for
      [LMS](https://datatracker.ietf.org/doc/html/rfc8554) key types.

### Changed

* Deprecated support for detached SCTs ([#188](https://github.com/sigstore/protobuf-specs/pull/188))

### Fixed

* Docs: Clarified rotation of verification materials in the trust root
  ([#210](https://github.com/sigstore/protobuf-specs/pull/210)

## 0.2.1

### Added

* CloudEvents proto for Rekor pub/sub messages ([#86](https://github.com/sigstore/protobuf-specs/pull/86))
* Generate jsonschema ([#112](https://github.com/sigstore/protobuf-specs/pull/112))
* Rust bindings for jsonschema ([#118](https://github.com/sigstore/protobuf-specs/pull/118))
* Dependabot to update dependencies ([#99](https://github.com/sigstore/protobuf-specs/pull/99))

### Changed

There were no changes in this release.

### Fixed

* Docs: Fixed spelling error ([#97](https://github.com/sigstore/protobuf-specs/pull/97))
* Docs: Clarified log index vs global log index ([#101](https://github.com/sigstore/protobuf-specs/pull/101))
* Docs: Clarified purpose of SET as a signed timestamp ([#100](https://github.com/sigstore/protobuf-specs/pull/100))
* Docs: Clarify message digest purpose ([#114](https://github.com/sigstore/protobuf-specs/pull/114))

### Removed

There were no removals in this release.

## 0.2.0

### Added

* Rust bindings have been added ([#88](https://github.com/sigstore/protobuf-specs/pull/88))

### Changed

* `TransparencyLogEntry.inclusion_proof` is now marked as required (was previously optional),
  while `TransparencyLogEntry.inclusion_promise` is now marked as optional (was previously
  required) ([#84](https://github.com/sigstore/protobuf-specs/pull/84))

* More Rekor messages and message fields have been marked as required
 ([#79](https://github.com/sigstore/protobuf-specs/pull/79))

* Ruby bindings: class names have been updated and now live in the `Sigstore::` namespace
  ([#87](https://github.com/sigstore/protobuf-specs/pull/87))

### Fixed

* Docs: Clarify that `TransparencyLogEntry.canonicalized_body` is optional
  ([#74](https://github.com/sigstore/protobuf-specs/pull/74))

* Docs: Clarify that key IDs are digests over SPKI encodings
  ([#73](https://github.com/sigstore/protobuf-specs/pull/73))

* Docs: Clarify that bundled certificate chains must not contain root or intermediate
  certificates that should be trused out-of-band
  ([#77](https://github.com/sigstore/protobuf-specs/pull/77))

* Docs: Clarify `TimeRange` validity periods
  ([#78](https://github.com/sigstore/protobuf-specs/pull/78))

### Removed

There were no removals in this release.
