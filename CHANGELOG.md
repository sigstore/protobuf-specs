# Changelog

All notable changes to `protobuf-specs` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

All versions prior to 0.2.0 are untracked.

## [Unreleased]

### Added

### Changed

## 0.4.2

### Changed

* Deprecated LMS/LMS-OTS as supported signing algorithms
  ([#597](https://github.com/sigstore/protobuf-specs/pull/597))
* Added stronger language around how clients should handle service selection
  ([#607](https://github.com/sigstore/protobuf-specs/pull/607))
* Revised checkpoint key ID comment, deprecated log ID
  ([#629](https://github.com/sigstore/protobuf-specs/pull/629))
* Added operator for SigningConfig services, log and TSA roots
  ([#634](https://github.com/sigstore/protobuf-specs/pull/634))
* Added ML-DSA to algorithm registry
  ([#616](https://github.com/sigstore/protobuf-specs/pull/616))

## 0.4.1

### Changed

* Updated SigningConfig to specify API versions and validity periods
  ([#539](https://github.com/sigstore/protobuf-specs/pull/539))
* Added deprecated, but still in use, algorithms for ECDSA P384 and P512
  using SHA256 ([#572](https://github.com/sigstore/protobuf-specs/pull/572))

## 0.4.0

* Announced deprecation of JSONSchema outputs from this project
  ([#493](https://github.com/sigstore/protobuf-specs/pull/493))

### Fixed

* Fix toolchain to write generated code as the user running the build instead of root
  ([#473](https://github.com/sigstore/protobuf-specs/pull/473))

### Changed

* Recreated toolchain for code generation instead of depending on third-party container image
  ([#469](https://github.com/sigstore/protobuf-specs/pull/469))
  ([#475](https://github.com/sigstore/protobuf-specs/pull/475))
* Updated code generation tools for Go library to latest stable releases
  ([#476](https://github.com/sigstore/protobuf-specs/pull/476))
* Updated code generation tools for JSONSchema files to latest stable releases
  ([#478](https://github.com/sigstore/protobuf-specs/pull/478))
* Updated code generation tools for Python library to latest stable releases
  ([#479](https://github.com/sigstore/protobuf-specs/pull/479))
* Updated code generation tools for Ruby library to latest stable releases
  ([#481](https://github.com/sigstore/protobuf-specs/pull/481))
* Updated code generation tools for Rust library to latest stable releases
  ([#486](https://github.com/sigstore/protobuf-specs/pull/486))
* Updated code generation tools for Typescript library to latest stable releases
  ([#488](https://github.com/sigstore/protobuf-specs/pull/488))

## 0.3.3

* Allowed specifying artifact digest for verification
  ([#406](https://github.com/sigstore/protobuf-specs/pull/406))
* Added version to `SigningConfig` message
  ([#383](https://github.com/sigstore/protobuf-specs/pull/383))

### Changed

* Docs: Clarify that integration time is only trustworthy with a Signed Entry
  Timestamp ([#442](https://github.com/sigstore/protobuf-specs/pull/442))
* Docs: Clarify inclusion promise requirement ([#380](https://github.com/sigstore/protobuf-specs/pull/380))
* Docs: Clarify that artifact digest verification should not be used with
  in-toto attestations
  ([#461](https://github.com/sigstore/protobuf-specs/pull/461))

## 0.3.2

* Added `TransparencyLogInstance.checkpoint_key_id` as an optional key identifier
  for logs that generate checkpoints ([#284](https://github.com/sigstore/protobuf-specs/pull/284))

### Changed

* Docs: Clarified DSSE envelope signature cardinality ([#318](https://github.com/sigstore/protobuf-specs/pull/318))
* Docs: Clarifier behavior of key identifiers ([#284](https://github.com/sigstore/protobuf-specs/pull/284))

## 0.3.1

* Added client configuration message for signing ([#277](https://github.com/sigstore/protobuf-specs/pull/277))
* Added a new format for the media type that is compatible with OCI registries ([#279](https://github.com/sigstore/protobuf-specs/pull/279))
* Added events.proto for Ruby package ([#264](https://github.com/sigstore/protobuf-specs/pull/264))
* Targeted Node16 for Typescript package ([#230](https://github.com/sigstore/protobuf-specs/pull/230))

### Changed

* Docs: Removed timestamp from checkpoint ([#247](https://github.com/sigstore/protobuf-specs/pull/247))
* Remove EXPERIMENTAL prefix from LMS schemes ([#214](https://github.com/sigstore/protobuf-specs/pull/214))

### Fixed

* Docs: Clarified trust anchor in chain ([#245]([https://github.com/sigstore/protobuf-specs/pull/210](https://github.com/sigstore/protobuf-specs/pull/245))

## 0.3.0

* Options for more generic observer time ([#179](https://github.com/sigstore/protobuf-specs/pull/179))
* **BREAKING**: `VerificationMaterials.contents` now has an additional `certificate` variant,
  which is preferred in `0.3` bundles with the Sigstore PGI ([#191](https://github.com/sigstore/protobuf-specs/pull/191))
* Added algorithm registry documentation and updated `PublicKeyDetails` message
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
