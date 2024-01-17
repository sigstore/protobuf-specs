# Algorithm Registry

This file is designed to act as a source of truth regarding what signing
algorithms are recommended across the Sigstore ecosystem. Any changes to this
file **must** be reflected in the `KnownSignatureAlgorithm` enumeration in
[sigstore_common.proto](../protos/sigstore_common.proto).

Note that Sigstore clients and services aren't required support all algorithms
in this registry and may support algorithms that aren't in the registry. The
algorithm registry is more of a guideline than a rule and is meant to serve as
a secure set of defaults that the community can follow.

Refer to the [Sigstore: Configurable Crypto Algorithms](https://docs.google.com/document/d/18vTKFvTQdRt3OGz6Qd1xf04o-hugRYSup-1EAOWn7MQ/)
specification for the design rationale for this registry.

| Algorithm | Name | Usage |
| --- | --- | --- |
| ECDSA | ecdsa-sha2-256-nistp256 | sign/verify |
|| ecdsa-sha2-384-nistp384 | sign/verify |
|| ecdsa-sha2-512-nistp521 | sign/verify |
| EdDSA | ed25519 | sign/verify |
|| ed25519-ph | sign/verify (recommended only for `hashedrekord`) |
