# protobuf-specs

This repository holds protobuf specifications for Sigstore messages.

## Protobuf

If you change protobuf definitions, you will need to regenerate the code by running the protocol buffer compiler on the changed `.proto` files.

You will need [Docker](https://docs.docker.com/get-docker/) installed to generate the protobuf stubs. Then run,

```
$ make all
```

to generate the Go and Python files under `gen/`.
