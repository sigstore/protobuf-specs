# Changelog

All notable changes to `protobuf-specs` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

All versions prior to 0.2.0 are untracked.

## [Unreleased]

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
