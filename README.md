# protobuf-specs

This repository holds protobuf specifications for Sigstore messages.

## Protobuf

If you change protobuf definitions, you will need to regenerate the code by running the protocol buffer compiler on the changed `.proto` files.

You will need [Go](https://www.golang.org/) installed on your machine. For local development, ensure that a [GOPATH](https://golang.org/doc/code.html#GOPATH) is set up. Then run, 

```
$ make proto
```

to generate the `*.pb.go` files under `protos/`.