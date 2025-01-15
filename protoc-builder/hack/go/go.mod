module github.com/sigstore/protobuf-specs/protoc-builder/hack/go

go 1.21

toolchain go1.23.4

require (
	google.golang.org/grpc/cmd/protoc-gen-go-grpc v1.5.1
	google.golang.org/protobuf v1.36.2
)

require golang.org/x/xerrors v0.0.0-20220907171357-04be3eba64a2 // indirect
