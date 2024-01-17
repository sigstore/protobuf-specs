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

## Hash Algorithms

| Algorithm | Name         |
|-----------|--------------|
| SHA2      | sha2-256     |
|           | sha2-256/192 |
|           | sha2-384     |
| SHA3      | sha3-256     |
|           | sha3-384     |

## Signature Algorithms

| Algorithm | Name                       | Usage                                             |
|-----------|----------------------------|---------------------------------------------------|
| RSA       | rsa-sign-pkcs1-2048-sha256 | verify only                                       |
|           | rsa-sign-pkcs1-3072-sha256 | sign/verify                                       |
|           | rsa-sign-pkcs1-4096-sha256 | sign/verify                                       |
| ECDSA     | ecdsa-sha2-256-nistp256    | sign/verify                                       |
|           | ecdsa-sha2-384-nistp384    | sign/verify                                       |
|           | ecdsa-sha2-512-nistp521    | sign/verify                                       |
| EdDSA     | ed25519                    | sign/verify                                       |
|           | ed25519-ph                 | sign/verify (recommended only for `hashedrekord`) |
