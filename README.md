# protobuf-specs

This repository holds protobuf specifications for Sigstore messages.

## Protobuf

If you change protobuf definitions, you will need to regenerate the code by running the protocol buffer compiler on the changed `.proto` files.

You will need [Docker](https://docs.docker.com/get-docker/) installed and configured to [run as non-root user](https://docs.docker.com/engine/install/linux-postinstall/#manage-docker-as-a-non-root-user) to generate the protobuf stubs. Then run,

```
$ make all
```

to generate the Go and Python files under `gen/`.

## Adding New Algorithms

With the standardization of post-quantum cryptography signing algorithms by NIST,
ML-DSA (FIPS 204, Dilithium) and SLH-DSA (FIPS 205, SPHINCS+), and with ongoing
work to standardize [another set of algorithms](https://csrc.nist.gov/projects/pqc-dig-sig),
Sigstore will be accepting additional algorithms to sign artifacts and verification material.

To add a new algorithm, you must first get consensus with the community through
an update to the
[algorithm registry specification](https://github.com/sigstore/architecture-docs/blob/main/algorithm-registry.md).
Tag client maintainers to make sure that the new algorithm can be supported by their ecosystem.
Algorithms do not have to be supported by all clients, but you should not propose an algorithm
that is not widely standardized. Algorithms must be supported in Go since Fulcio and Rekor
will need to be updated to support signature verification, and the Go libraries should be
well-known and vetted and not based on C implementations with Go bindings.

After updating the specification, update the
[`PublicKeyDetails`](https://github.com/sigstore/protobuf-specs/blob/c30eb14cece57d88c08579197ecfdb57a5f1aba5/protos/sigstore_common.proto#L63)
to include the new signing algorithm identifier. If the algorithm also uses a new hashing algorithm, update
[`HashAlgorithm`](https://github.com/sigstore/protobuf-specs/blob/c30eb14cece57d88c08579197ecfdb57a5f1aba5/protos/sigstore_common.proto#L37).

## Service Builder

This project publishes a container to [`ghcr.io/sigstore/protobuf-specs-service-builder`](https://github.com/sigstore/protobuf-specs/pkgs/container/protobuf-specs-service-builder)
which contains all the necessary protoc tools, .proto files and .proto dependencies to generate service 
defintions for sigstore services (like rekor and fulcio). This container is not meant to be used by anyone
else and no requests or support will be provided.

## Deprecation Notice

- Effective July 17th, 2025: the jsonschema generated files in gen/jsonschema/schemas/ were removed from this repository and not built repeatedly going forward.
