module github.com/sigstore/protobuf-specs/protoc-builder/hack/go

go 1.24

toolchain go1.24.0

tool (
	google.golang.org/grpc/cmd/protoc-gen-go-grpc
	google.golang.org/protobuf/cmd/protoc-gen-go
)

require (
	google.golang.org/grpc/cmd/protoc-gen-go-grpc v1.5.1 // indirect
	google.golang.org/protobuf v1.36.5 // indirect
)
