# protobuf-specs

This repository holds protobuf specifications for Sigstore messages.

## Protobuf

If you change protobuf definitions, you will need to regenerate the code by running the protocol buffer compiler on the changed `.proto` files.

You will need [Docker](https://docs.docker.com/get-docker/) installed to generate the protobuf stubs. Then run, 

```
$ make proto
```

to generate the `*.pb.go` files under `gen/`.