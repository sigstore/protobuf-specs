# protobuf-specs

This repository holds protobuf specifications for Sigstore messages.

## Protobuf

If you change protobuf definitions, you will need to regenerate the code by running the protocol buffer compiler on the changed `.proto` files.

You will need [Docker](https://docs.docker.com/get-docker/) installed and configured to [run as non-root user](https://docs.docker.com/engine/install/linux-postinstall/#manage-docker-as-a-non-root-user) to generate the protobuf stubs. Then run,

```
$ make all
```

to generate the Go and Python files under `gen/`.

## Deprecation Notice

- Effective January 17th, 2025: the jsonschema generated files in gen/jsonschema/schemas/ are formally deprecated. They will be removed in 6 months from this repository and not built repeatedly going forward. If you are using them, please open an issue on this repository and let us know.
